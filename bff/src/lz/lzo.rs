use std::io::Write;
use std::ptr::null_mut;

use absperf_minilzo_sys::{lzo1x_1_compress, lzo1x_decompress_safe, lzo_uint};

use crate::BffResult;

pub fn lzo_compress<W: Write>(data: &[u8], writer: &mut W) -> BffResult<()> {
    let mut compressed = Vec::with_capacity(data.len() + data.len() / 16 + 64 + 3);

    unsafe {
        let mut compressed_len = 0;
        let mut wrkmem = [0usize; 0x4000];

        let result = lzo1x_1_compress(
            data.as_ptr(),
            data.len().try_into().unwrap(),
            compressed.as_mut_ptr(),
            &mut compressed_len,
            wrkmem.as_mut_ptr() as *mut _,
        );
        assert_eq!(result, 0);

        compressed.set_len(compressed_len as usize);
    }

    writer.write_all(&compressed)?;
    Ok(())
}

pub fn lzo_decompress(compressed: &[u8], decompressed_buffer_size: usize) -> BffResult<Vec<u8>> {
    let mut decompressed = Vec::with_capacity(decompressed_buffer_size);

    unsafe {
        let mut decompressed_len = decompressed_buffer_size as lzo_uint;

        let result = lzo1x_decompress_safe(
            compressed.as_ptr(),
            compressed.len().try_into().unwrap(),
            decompressed.as_mut_ptr(),
            &mut decompressed_len,
            null_mut(),
        );
        assert_eq!(result, 0);

        decompressed.set_len(decompressed_len as usize);
    }

    Ok(decompressed)
}
