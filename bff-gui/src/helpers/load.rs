use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use bff::bigfile::BigFile;
use bff::platforms::Platform;

pub fn load_bf(path: PathBuf, tx: Sender<(BigFile, PathBuf)>) {
    tokio::spawn(async move {
        let platform = match path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(&path).unwrap();
        let mut reader = bff::BufReader::new(f);
        let bf = BigFile::read_platform(&mut reader, platform).unwrap();
        let _ = tx.send((bf, path));
    });
}
