use std::{fs::File, path::PathBuf, sync::mpsc::Sender};

use bff::{bigfile::BigFile, platforms::Platform};

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
