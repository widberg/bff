#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use bff::bigfile::BigFile;
    use bff::platforms::Platform;
    use binrw::io::BufReader;
    use test_generator::test_resources;

    #[test_resources("data/bigfiles/**/*.*")]
    fn read(bigfile_path_str: &str) {
        let mut bigfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bigfile_path.pop();
        bigfile_path.push(bigfile_path_str);
        let platform = match bigfile_path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(bigfile_path).unwrap();
        let mut reader = BufReader::new(f);
        let _ = BigFile::read_platform(&mut reader, platform).unwrap();
    }
}
