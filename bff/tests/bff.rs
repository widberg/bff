#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use bff::bigfile::BigFile;
    use bff::platforms::extension_to_endian;
    use binrw::io::BufReader;
    use binrw::Endian;
    use test_generator::test_resources;

    #[test_resources("data/bigfiles/**/*.*")]
    fn read(bigfile_path_str: &str) {
        let mut bigfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bigfile_path.pop();
        bigfile_path.push(bigfile_path_str);
        let endian = match bigfile_path.extension() {
            Some(extension) => extension_to_endian(extension).unwrap_or(Endian::Little),
            None => Endian::Little,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let _ = BigFile::read_endian(&mut reader, endian).unwrap();
    }
}
