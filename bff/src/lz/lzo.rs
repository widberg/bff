use std::io::Write;

use crate::BffResult;

pub fn lzo_compress<W: Write>(data: &[u8], writer: &mut W) -> BffResult<()> {
    let compressed = compress(data);

    writer.write_all(&compressed)?;
    Ok(())
}

pub fn lzo_decompress(compressed: &[u8], decompressed_buffer_size: usize) -> BffResult<Vec<u8>> {
    let decompressed = decompress(compressed, decompressed_buffer_size)?;

    Ok(decompressed)
}

// `LZO1X-1` real-time compression and safe decompression, in pure safe Rust.
// Compatible with minilzo-2.02.

/// Errors returned by [`decompress`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzoError {
    /// The compressed input ended before the decoder expected more bytes.
    InputOverrun,
    /// Decoding would write past the caller-provided output capacity.
    OutputOverrun,
    /// A back-reference pointed before the start of the output.
    LookbehindOverrun,
    /// The end-of-stream marker was never reached.
    EofNotFound,
    /// Decoding finished but input bytes remained.
    InputNotConsumed,
}

impl core::fmt::Display for LzoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::InputOverrun => "input overrun",
            Self::OutputOverrun => "output overrun",
            Self::LookbehindOverrun => "lookbehind overrun",
            Self::EofNotFound => "end-of-stream marker not found",
            Self::InputNotConsumed => "input not fully consumed",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for LzoError {}

// --- LZO1X stream constants ---

const M2_MAX_LEN: usize = 8;
const M3_MAX_LEN: usize = 33;
const M4_MAX_LEN: usize = 9;

const M2_MAX_OFFSET: usize = 0x0800;
const M3_MAX_OFFSET: usize = 0x4000;
const M4_MAX_OFFSET: usize = 0xbfff;

const M3_MARKER: usize = 32;
const M4_MARKER: usize = 16;

// Compressor hash dictionary: 2^14 entries.
const D_BITS: u32 = 14;
const D_SIZE: usize = 1 << D_BITS;
const D_MASK: u32 = (D_SIZE as u32) - 1;
const D_HIGH: u32 = (D_MASK >> 1) + 1;

/// Sentinel for an empty dictionary slot.
const NIL: usize = usize::MAX;

// --- Compression ---

/// Primary hash of the four bytes at `src[p..]`.
#[inline]
const fn d_index1(src: &[u8], p: usize) -> usize {
    let p0 = src[p] as u32;
    let p1 = src[p + 1] as u32;
    let p2 = src[p + 2] as u32;
    let p3 = src[p + 3] as u32;
    let h = (((p3 << 6) ^ p2) << 5) ^ p1;
    let h = (h << 5) ^ p0;
    // Wrapping 32-bit multiply.
    ((h.wrapping_mul(0x21) >> 5) & D_MASK) as usize
}

/// Secondary hash index derived from the primary one.
#[inline]
const fn d_index2(d: usize) -> usize {
    ((d as u32 & (D_MASK & 0x7ff)) ^ (D_HIGH | 0x1f)) as usize
}

/// Validate a dictionary hit, returning the back-reference offset if the stored
/// position is a usable match.
#[inline]
const fn check_mpos(m_pos: usize, ip: usize) -> Option<usize> {
    if m_pos == NIL {
        return None;
    }
    // Stored positions are always strictly less than `ip`.
    let m_off = ip - m_pos;
    if m_off == 0 || m_off > M4_MAX_OFFSET {
        None
    } else {
        Some(m_off)
    }
}

#[inline]
fn emit_m3_m4_offset(out: &mut Vec<u8>, m_off: usize) {
    out.push(((m_off & 63) << 2) as u8);
    out.push((m_off >> 6) as u8);
}

#[inline]
fn emit_m3_m4_len(out: &mut Vec<u8>, mut m_len: usize) {
    while m_len > 255 {
        m_len -= 255;
        out.push(0);
    }
    out.push(m_len as u8);
}

/// The main matching loop. Appends encoded data to `out` and returns the number
/// of trailing literal bytes not yet emitted. Only called when
/// `src.len() > M2_MAX_LEN + 5`.
fn do_compress(src: &[u8], out: &mut Vec<u8>) -> usize {
    let in_len = src.len();
    let in_end = in_len;
    let ip_end = in_len - M2_MAX_LEN - 5;

    let mut dict = vec![NIL; D_SIZE];

    // `ip` is the current input position; `ii` the start of the pending run of
    // unencoded literals. The first four bytes are always emitted as literals.
    let mut ip: usize = 4;
    let mut ii: usize = 0;

    loop {
        let h1 = d_index1(src, ip);
        let mut dindex = h1;
        let mut m_pos = dict[dindex];
        let mut m_off = 0usize;

        // Probe up to two hash slots for a usable match at the current position.
        let go_literal: bool = 'probe: {
            match check_mpos(m_pos, ip) {
                None => break 'probe true,
                Some(off) => m_off = off,
            }
            if !(m_off <= M2_MAX_OFFSET || src[m_pos + 3] == src[ip + 3]) {
                dindex = d_index2(h1);
                m_pos = dict[dindex];
                match check_mpos(m_pos, ip) {
                    None => break 'probe true,
                    Some(off) => m_off = off,
                }
                if !(m_off <= M2_MAX_OFFSET || src[m_pos + 3] == src[ip + 3]) {
                    break 'probe true;
                }
            }
            // Confirm the first three bytes actually match.
            if src[m_pos] != src[ip] || src[m_pos + 1] != src[ip + 1] {
                break 'probe true;
            }
            if src[m_pos + 2] == src[ip + 2] {
                break 'probe false;
            }
            true
        };

        if go_literal {
            dict[dindex] = ip;
            ip += 1;
            if ip >= ip_end {
                break;
            }
            continue;
        }

        dict[dindex] = ip;

        // Emit the pending literal run [ii, ip).
        let t = ip - ii;
        if t > 0 {
            if t <= 3 {
                let l = out.len();
                out[l - 2] |= t as u8;
            } else if t <= 18 {
                out.push((t - 3) as u8);
            } else {
                let mut tt = t - 18;
                out.push(0);
                while tt > 255 {
                    tt -= 255;
                    out.push(0);
                }
                out.push(tt as u8);
            }
            for _ in 0..t {
                out.push(src[ii]);
                ii += 1;
            }
        }

        // Determine match length. Compare m_pos[3..=8] with ip[3..], where ip
        // advances by 3 first (the first three bytes already matched).
        ip += 3;
        let mut mismatch = false;
        for k in 3..=8usize {
            let b = src[ip];
            ip += 1;
            if src[m_pos + k] != b {
                mismatch = true;
                break;
            }
        }

        if mismatch {
            ip -= 1;
            let m_len = ip - ii; // 3..=8

            if m_off <= M2_MAX_OFFSET {
                m_off -= 1;
                out.push((((m_len - 1) << 5) | ((m_off & 7) << 2)) as u8);
                out.push((m_off >> 3) as u8);
            } else if m_off <= M3_MAX_OFFSET {
                m_off -= 1;
                out.push((M3_MARKER | (m_len - 2)) as u8);
                emit_m3_m4_offset(out, m_off);
            } else {
                m_off -= 0x4000;
                out.push((M4_MARKER | ((m_off & 0x4000) >> 11) | (m_len - 2)) as u8);
                emit_m3_m4_offset(out, m_off);
            }
        } else {
            // Extend the match past M2_MAX_LEN.
            let mut m = m_pos + M2_MAX_LEN + 1;
            while ip < in_end && src[m] == src[ip] {
                m += 1;
                ip += 1;
            }
            let mut ml = ip - ii;

            if m_off <= M3_MAX_OFFSET {
                m_off -= 1;
                if ml <= M3_MAX_LEN {
                    out.push((M3_MARKER | (ml - 2)) as u8);
                } else {
                    ml -= M3_MAX_LEN;
                    out.push(M3_MARKER as u8);
                    emit_m3_m4_len(out, ml);
                }
            } else {
                m_off -= 0x4000;
                if ml <= M4_MAX_LEN {
                    out.push((M4_MARKER | ((m_off & 0x4000) >> 11) | (ml - 2)) as u8);
                } else {
                    ml -= M4_MAX_LEN;
                    out.push((M4_MARKER | ((m_off & 0x4000) >> 11)) as u8);
                    emit_m3_m4_len(out, ml);
                }
            }
            emit_m3_m4_offset(out, m_off);
        }

        ii = ip;
        if ip >= ip_end {
            break;
        }
    }

    in_end - ii
}

/// Compress `src` with `LZO1X-1`, returning the compressed bytes.
///
/// Compression never fails.
pub fn compress(src: &[u8]) -> Vec<u8> {
    let in_len = src.len();
    let mut out: Vec<u8> = Vec::with_capacity(in_len + in_len / 16 + 64 + 3);

    let mut t = if in_len <= M2_MAX_LEN + 5 {
        in_len
    } else {
        do_compress(src, &mut out)
    };

    if t > 0 {
        let ii = in_len - t; // start of trailing literals
        if out.is_empty() && t <= 238 {
            out.push((17 + t) as u8);
        } else if t <= 3 {
            let l = out.len();
            out[l - 2] |= t as u8;
        } else if t <= 18 {
            out.push((t - 3) as u8);
        } else {
            let mut tt = t - 18;
            out.push(0);
            while tt > 255 {
                tt -= 255;
                out.push(0);
            }
            out.push(tt as u8);
        }
        for k in 0..t {
            out.push(src[ii + k]);
        }
        t = 0;
    }
    let _ = t;

    // End-of-stream marker.
    out.push((M4_MARKER | 1) as u8);
    out.push(0);
    out.push(0);

    out
}

// --- Decompression ---

/// Safely decompress an `LZO1X` stream.
///
/// `dst_capacity` is the maximum number of output bytes to produce. The returned
/// vector's length is the actual decompressed size, which may be smaller than
/// `dst_capacity`. Decoding never reads or writes out of bounds; a malformed or
/// truncated stream yields an [`LzoError`].
pub fn decompress(src: &[u8], dst_capacity: usize) -> Result<Vec<u8>, LzoError> {
    let ip_end = src.len();
    let mut out: Vec<u8> = Vec::with_capacity(dst_capacity);
    let mut ip: usize = 0;

    // Fail unless at least `x` input bytes remain.
    macro_rules! need_ip {
        ($x:expr) => {
            if ip_end - ip < $x {
                return Err(LzoError::InputOverrun);
            }
        };
    }
    // Fail unless at least `x` bytes of output capacity remain.
    macro_rules! need_op {
        ($x:expr) => {
            if dst_capacity - out.len() < $x {
                return Err(LzoError::OutputOverrun);
            }
        };
    }

    if src.is_empty() {
        return Err(LzoError::InputOverrun);
    }

    let mut t: usize;

    #[derive(Clone, Copy)]
    enum State {
        Top,
        FirstLiteralRun,
        Match,
        MatchDone,
        MatchNext,
    }

    // A back-reference position within `out` (index of the source byte);
    // assigned in each match arm before use.
    let mut m_pos: usize;

    // Preamble.
    let mut state;
    if src[ip] as usize > 17 {
        t = src[ip] as usize - 17;
        ip += 1;
        if t < 4 {
            state = State::MatchNext;
        } else {
            need_op!(t);
            need_ip!(t + 1);
            for _ in 0..t {
                out.push(src[ip]);
                ip += 1;
            }
            state = State::FirstLiteralRun;
        }
    } else {
        t = 0;
        state = State::Top;
    }

    loop {
        state = match state {
            State::Top => {
                if ip >= ip_end {
                    // Ran out of input before the end-of-stream marker.
                    return Err(LzoError::EofNotFound);
                }
                t = src[ip] as usize;
                ip += 1;
                if t >= 16 {
                    State::Match
                } else {
                    if t == 0 {
                        need_ip!(1);
                        while src[ip] == 0 {
                            t += 255;
                            ip += 1;
                            need_ip!(1);
                        }
                        t += 15 + src[ip] as usize;
                        ip += 1;
                    }
                    // Copy t + 3 literal bytes.
                    need_op!(t + 3);
                    need_ip!(t + 4);
                    for _ in 0..t + 3 {
                        out.push(src[ip]);
                        ip += 1;
                    }
                    State::FirstLiteralRun
                }
            }

            State::FirstLiteralRun => {
                t = src[ip] as usize;
                ip += 1;
                if t >= 16 {
                    State::Match
                } else {
                    let off = (1 + M2_MAX_OFFSET) + (t >> 2) + ((src[ip] as usize) << 2);
                    ip += 1;
                    if off > out.len() {
                        return Err(LzoError::LookbehindOverrun);
                    }
                    m_pos = out.len() - off;
                    need_op!(3);
                    for _ in 0..3 {
                        let b = out[m_pos];
                        out.push(b);
                        m_pos += 1;
                    }
                    State::MatchDone
                }
            }

            State::Match => {
                if t >= 64 {
                    let off = 1 + ((t >> 2) & 7) + ((src[ip] as usize) << 3);
                    ip += 1;
                    if off > out.len() {
                        return Err(LzoError::LookbehindOverrun);
                    }
                    m_pos = out.len() - off;
                    t = (t >> 5) - 1;
                    copy_match(&mut out, &mut m_pos, t, dst_capacity)?;
                } else if t >= 32 {
                    t &= 31;
                    if t == 0 {
                        need_ip!(1);
                        while src[ip] == 0 {
                            t += 255;
                            ip += 1;
                            need_ip!(1);
                        }
                        t += 31 + src[ip] as usize;
                        ip += 1;
                    }
                    let off = 1 + ((src[ip] as usize >> 2) + ((src[ip + 1] as usize) << 6));
                    ip += 2;
                    if off > out.len() {
                        return Err(LzoError::LookbehindOverrun);
                    }
                    m_pos = out.len() - off;
                    copy_match(&mut out, &mut m_pos, t, dst_capacity)?;
                } else if t >= 16 {
                    let mut base = (t & 8) << 11;
                    t &= 7;
                    if t == 0 {
                        need_ip!(1);
                        while src[ip] == 0 {
                            t += 255;
                            ip += 1;
                            need_ip!(1);
                        }
                        t += 7 + src[ip] as usize;
                        ip += 1;
                    }
                    base += (src[ip] as usize >> 2) + ((src[ip + 1] as usize) << 6);
                    ip += 2;
                    if base == 0 {
                        // A zero offset here is the end-of-stream marker.
                        return finish(&mut out, ip, ip_end);
                    }
                    let off = base + 0x4000;
                    if off > out.len() {
                        return Err(LzoError::LookbehindOverrun);
                    }
                    m_pos = out.len() - off;
                    copy_match(&mut out, &mut m_pos, t, dst_capacity)?;
                } else {
                    // Length-2 match.
                    let off = 1 + (t >> 2) + ((src[ip] as usize) << 2);
                    ip += 1;
                    if off > out.len() {
                        return Err(LzoError::LookbehindOverrun);
                    }
                    m_pos = out.len() - off;
                    need_op!(2);
                    let b0 = out[m_pos];
                    out.push(b0);
                    let b1 = out[m_pos + 1];
                    out.push(b1);
                }

                State::MatchDone
            }

            State::MatchDone => {
                // The low two bits of the previous control byte are the number
                // of literal bytes that follow this match.
                t = src[ip - 2] as usize & 3;
                if t == 0 { State::Top } else { State::MatchNext }
            }

            State::MatchNext => {
                need_op!(t);
                need_ip!(t + 1);
                out.push(src[ip]);
                ip += 1;
                if t > 1 {
                    out.push(src[ip]);
                    ip += 1;
                    if t > 2 {
                        out.push(src[ip]);
                        ip += 1;
                    }
                }
                t = src[ip] as usize;
                ip += 1;
                // Stop if the input is exhausted before the next token.
                if ip >= ip_end {
                    return Err(LzoError::EofNotFound);
                }
                State::Match
            }
        };
    }
}

/// Emit a match of length `t + 2` from `m_pos`, one byte at a time so that
/// overlapping (run-length) copies work.
#[inline]
fn copy_match(
    out: &mut Vec<u8>,
    m_pos: &mut usize,
    t: usize,
    dst_capacity: usize,
) -> Result<(), LzoError> {
    // Callers already validated the back-reference; check output capacity here.
    if dst_capacity - out.len() < t + 2 {
        return Err(LzoError::OutputOverrun);
    }
    for _ in 0..t + 2 {
        let b = out[*m_pos];
        out.push(b);
        *m_pos += 1;
    }
    Ok(())
}

/// Finalize at the end-of-stream marker, validating that the input was fully
/// consumed. The marker's two bytes are always read in-bounds before reaching
/// here, so `ip` can never exceed `ip_end`; only the equal / short cases occur.
#[inline]
fn finish(out: &mut Vec<u8>, ip: usize, ip_end: usize) -> Result<Vec<u8>, LzoError> {
    let result = std::mem::take(out);
    if ip == ip_end {
        Ok(result)
    } else {
        Err(LzoError::InputNotConsumed)
    }
}
