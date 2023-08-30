use std::cmp::{max, min};
use std::io::SeekFrom;
use std::ptr::null_mut;

use binrw::{BinReaderExt, BinResult, BinWriterExt};

#[binrw::parser(reader, endian)]
pub fn decompress_body_parser(decompressed_size: u32, compressed_size: u32) -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let read_decompressed_size = reader.read_le::<u32>()?;
    let read_compressed_size = reader.read_le::<u32>()?;

    // Ensure the values from the object header match the values
    // in the compressed data.
    // compressed_size includes the 8 bytes taken up by the duplicate
    // size fields.
    assert_eq!(decompressed_size, read_decompressed_size);
    assert_eq!(compressed_size, read_compressed_size);

    decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::parser(reader, endian)]
pub fn decompress_data_with_header_parser() -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let decompressed_size = reader.read_le::<u32>()?;
    let compressed_size = reader.read_le::<u32>()?;

    decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::parser(reader)]
pub fn decompress_data_parser(decompressed_size: u32, _compressed_size: u32) -> BinResult<Vec<u8>> {
    const WINDOW_LOG: u16 = 14;
    const WINDOW_MASK: u16 = (1 << WINDOW_LOG) - 1;

    let mut decompressed_buffer: Vec<u8> = Vec::new();

    loop {
        let mut flags = reader.read_be::<u32>()?;
        let len = (flags & 0b11) as u16;
        let temp_shift = WINDOW_LOG - len;
        let temp_mask = WINDOW_MASK >> len;

        for _ in 0..30 {
            if (flags & 0x80000000) != 0 {
                let temp = reader.read_be::<u16>()?;
                let start = decompressed_buffer.len() - (temp & temp_mask) as usize - 1;
                let end = start + (temp >> temp_shift) as usize + 3;

                for i in start..end {
                    decompressed_buffer.push(decompressed_buffer[i]);
                }
            } else {
                decompressed_buffer.push(reader.read_be::<u8>()?);
            }

            if decompressed_buffer.len() >= decompressed_size as usize {
                return Ok(decompressed_buffer);
            }

            flags <<= 1;
        }
    }
}

#[binrw::writer(writer, endian)]
pub fn compress_data_with_header_writer(data: &[u8]) -> BinResult<()> {
    // println!("{:?}", data);
    let starting_position = writer.stream_position()?;
    let decompressed_size = data.len() as u32;
    writer.write_le::<u32>(&decompressed_size)?;
    writer.write_le::<u32>(&0)?;

    compress_data_writer(data, writer, endian, ())?;
    let ending_position = writer.stream_position()?;

    let compressed_size = (ending_position - starting_position) as u32;
    writer.seek(SeekFrom::Start(4))?;
    writer.write_le::<u32>(&compressed_size)?;
    writer.seek(SeekFrom::Start(ending_position))?;
    Ok(())
}

// TODO: Make the lz compression code pretty and safe

#[derive(Clone)]
struct PacketMatch {
    length: i32,
    data: i32,
}

#[derive(Clone)]
struct Packet {
    match_length: i32,
    total_length: i32,
    matches: Vec<PacketMatch>,
}

impl Packet {
    fn with_match_length(match_length: i32) -> Self {
        Self {
            match_length,
            total_length: 0,
            matches: Vec::new(),
        }
    }

    fn reset_total_length(&mut self) {
        self.total_length = 0;
    }
}

// Oops I made a doubly linked list
#[derive(Clone)]
struct Match {
    pos: u64,
    prev: *mut Match,
    next: *mut Match,
}

impl Default for Match {
    fn default() -> Self {
        Self {
            pos: 0,
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

impl Match {
    // Detach self from previous
    unsafe fn split_off(&mut self) {
        self.prev.as_mut().unwrap().next = null_mut();
        self.prev = null_mut();
    }

    // Insert node between self and self.prev
    unsafe fn insert_before(&mut self, node: *mut Match) {
        node.as_mut().unwrap().prev = self.prev;
        if !node.as_mut().unwrap().prev.is_null() {
            node.as_mut().unwrap().prev.as_mut().unwrap().next = node;
        };
        node.as_mut().unwrap().next = self;
        node.as_mut().unwrap().next.as_mut().unwrap().prev = node;
    }
}

const MAXIMUM_WINDOW_SIZE: u32 = 0x8000;

fn encode_packet(
    mut uncompressed_buffer_ptr: u64,
    packet: &mut Packet,
    mut window_index: u32,
    uncompressed_buffer: &[u8],
    uncompressed_buffer_size: usize,
    g_window_buffer: &[Match],
) -> bool {
    let mut remaining_length: u32 = (1 << packet.match_length) + 2;
    let v20: u32 = 0x10000 >> packet.match_length;

    packet.matches.clear();
    for _ in 0..30 {
        let v5: u64 = max(0i64, uncompressed_buffer_ptr as i64 - v20 as i64) as u64;

        remaining_length = min(
            remaining_length,
            (uncompressed_buffer_size - uncompressed_buffer_ptr as usize) as u32,
        );

        if remaining_length <= 2 {
            packet.total_length += 1;
            packet.matches.push(PacketMatch {
                length: -1,
                data: uncompressed_buffer[uncompressed_buffer_ptr as usize] as i32,
            });
            uncompressed_buffer_ptr += 1;
            window_index += 1;
        } else {
            let mut ptr: u64 = 0;

            let mut match_length: i32 = 2;
            let mut cur: *const Match = g_window_buffer[window_index as usize].prev;
            unsafe {
                while !cur.is_null() && cur.as_ref().unwrap().pos >= v5 {
                    if uncompressed_buffer[uncompressed_buffer_ptr as usize + 2]
                        == uncompressed_buffer[cur.as_ref().unwrap().pos as usize + 2]
                    {
                        let mut j: i32 = 3;
                        while uncompressed_buffer[cur.as_ref().unwrap().pos as usize + j as usize]
                            == uncompressed_buffer[uncompressed_buffer_ptr as usize + j as usize]
                            && remaining_length != j as u32
                        {
                            j += 1;
                        }

                        if match_length < j {
                            if remaining_length == j as u32 {
                                ptr = cur.as_ref().unwrap().pos;
                                match_length = remaining_length as i32;
                                break;
                            }
                            match_length = j;
                            ptr = cur.as_ref().unwrap().pos;
                        }
                    }
                    cur = cur.as_ref().unwrap().prev;
                }
            }

            if match_length == 2 {
                packet.total_length += 1;
                packet.matches.push(PacketMatch {
                    length: -1,
                    data: uncompressed_buffer[uncompressed_buffer_ptr as usize] as i32,
                });
                uncompressed_buffer_ptr += 1;
                window_index += 1;
            } else {
                packet.total_length += match_length;
                packet.matches.push(PacketMatch {
                    length: match_length - 3,
                    data: uncompressed_buffer_ptr as i32 - ptr as i32,
                });
                uncompressed_buffer_ptr += match_length as u64;
                window_index += match_length as u32;
            }
        }

        window_index %= MAXIMUM_WINDOW_SIZE;

        if uncompressed_buffer_ptr >= uncompressed_buffer_size as u64 {
            return true;
        }
    }

    false
}

#[binrw::writer(writer)]
pub fn compress_data_writer(data: &[u8]) -> BinResult<()> {
    let uncompressed_buffer_size = data.len();
    let mut uncompressed_buffer = data.to_vec();
    uncompressed_buffer.push(0);
    uncompressed_buffer.push(0);
    let uncompressed_buffer = uncompressed_buffer.as_slice();

    assert_eq!(uncompressed_buffer.len(), uncompressed_buffer_size + 2);

    let mut g_window_buffer = vec![Match::default(); MAXIMUM_WINDOW_SIZE as usize];
    let mut short_lookup = vec![Match::default(); 0x10000];
    // I wish there was a cleaner way to do this at compile time.
    // https://stackoverflow.com/q/26757355/3997768
    let mut packets = [
        Packet::with_match_length(2),
        Packet::with_match_length(3),
        Packet::with_match_length(4),
        Packet::with_match_length(5),
    ];

    let window_size: u32 = min(uncompressed_buffer_size as u32, MAXIMUM_WINDOW_SIZE);
    let mut window_index = 0u32;

    for i in 0..window_size {
        let match_index: u16 = u16::from_be_bytes(
            uncompressed_buffer[i as usize..i as usize + 2]
                .try_into()
                .unwrap(),
        );
        let current: *mut Match = &mut g_window_buffer[i as usize];
        let next: *mut Match = &mut short_lookup[match_index as usize];
        unsafe {
            current.as_mut().unwrap().pos = i as u64;
            next.as_mut().unwrap().insert_before(current);
        }
    }

    let mut uncompressed_buffer_ptr = 0u64;

    let mut buffer_size_2: u32 = MAXIMUM_WINDOW_SIZE;
    let mut k: i32 = 0x7000;

    while uncompressed_buffer_ptr < uncompressed_buffer_size as u64 {
        packets.iter_mut().for_each(|p| p.reset_total_length());

        // If hashes don't match, this is probably where the problem is.
        // I hand re-rolled this loop to make it easier to read.
        // for loops aren't allowed to return values via break so I used find_map.
        // https://rust-lang.github.io/rfcs/1624-loop-break-value.html#extension-to-for-while-while-let
        // https://users.rust-lang.org/t/how-to-return-value-from-for-loop/79225/2
        let mut len = 3;
        let limits = [0, 180, 300, 540];
        let len = (0i32..=3)
            .rev()
            .find_map(|i| {
                if encode_packet(
                    uncompressed_buffer_ptr,
                    &mut packets[i as usize],
                    window_index,
                    uncompressed_buffer,
                    uncompressed_buffer_size,
                    &g_window_buffer,
                ) {
                    return Some(i);
                }

                if packets[i as usize].total_length > packets[len as usize].total_length
                    || (i == 0
                        && packets[i as usize].total_length >= packets[len as usize].total_length)
                {
                    len = i;
                }

                if packets[len as usize].total_length > limits[i as usize] {
                    return Some(len);
                }

                None
            })
            .unwrap_or(len);

        let current_packet: &Packet = &packets[len as usize];

        let mut flag: u32 = 0;
        for i in 0..current_packet.matches.len() {
            if current_packet.matches[i].length >= 0 {
                flag |= 0x80000000u32 >> i;
            }
        }

        writer.write_be::<u32>(&(flag | len as u32))?;

        for m in current_packet.matches.iter() {
            if m.length == -1 {
                writer.write_be::<u8>(&(m.data as u8))?;
            } else {
                writer.write_be::<u16>(&((m.data + (m.length << (0xE - len)) - 1) as u16))?;
            }
        }

        uncompressed_buffer_ptr += current_packet.total_length as u64;

        window_index = (window_index + current_packet.total_length as u32) % MAXIMUM_WINDOW_SIZE;

        k -= current_packet.total_length;
        if k < 0 {
            let window_size_1: u32 =
                min(uncompressed_buffer_size as u32, buffer_size_2 + 0x1000u32);
            for i in buffer_size_2..window_size_1 {
                let ptr: u32 = i;
                let match_index: u16 = u16::from_be_bytes(
                    uncompressed_buffer[ptr as usize..ptr as usize + 2]
                        .try_into()
                        .unwrap(),
                );
                let current: *mut Match = &mut g_window_buffer[(i % MAXIMUM_WINDOW_SIZE) as usize];
                let next: *mut Match = &mut short_lookup[match_index as usize];

                unsafe {
                    current.as_mut().unwrap().next.as_mut().unwrap().split_off();
                    current.as_mut().unwrap().pos = ptr as u64;
                    next.as_mut().unwrap().insert_before(current);
                }
            }
            k += 0x1000i32;
            buffer_size_2 = window_size_1;
        }
    }

    Ok(())
}
