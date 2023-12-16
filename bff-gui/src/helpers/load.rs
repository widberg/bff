use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;

pub fn load_bf(ctx: egui::Context, path: PathBuf, tx: Sender<Option<(BigFile, PathBuf)>>) {
    tokio::spawn(async move {
        let platform = match path.extension() {
            Some(extension) => extension.try_into().unwrap_or(Platform::PC),
            None => Platform::PC,
        };
        let f = File::open(&path).unwrap();
        let mut reader = bff::BufReader::new(f);
        match BigFile::read_platform(&mut reader, platform) {
            Ok(bf) => {
                let _ = tx.send(Some((bf, path)));
            }
            Err(e) => {
                println!("{}", e);
                let _ = tx.send(None);
            }
        }
        ctx.request_repaint();
    });
}
