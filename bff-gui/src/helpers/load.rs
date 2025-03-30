use std::path::PathBuf;
use std::sync::mpsc::Sender;

use bff::bigfile::platforms::Platform;
use bff::bigfile::BigFile;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_bf(ctx: egui::Context, path: PathBuf, tx: Sender<Option<(BigFile, PathBuf)>>) {
    use std::fs::File;
    tokio::spawn(async move {
        let platform = path
            .extension()
            .and_then(|e| e.try_into().ok())
            .unwrap_or(Platform::PC);
        let f = File::open(&path).unwrap();
        let mut reader = bff::BufReader::new(f);
        match BigFile::read_platform(&mut reader, platform, &None) {
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

#[cfg(target_arch = "wasm32")]
pub fn load_bf(
    ctx: egui::Context,
    file_name: String,
    data: Vec<u8>,
    tx: Sender<Option<(BigFile, PathBuf)>>,
) {
    use std::ffi::OsStr;

    let platform = file_name
        .rsplit_once(".")
        .and_then(|e| OsStr::new(e.1).try_into().ok())
        .unwrap_or(Platform::PC);
    let mut reader = bff::BufReader::new(std::io::Cursor::new(data));
    match BigFile::read_platform(&mut reader, platform, &None) {
        Ok(bf) => {
            let _ = tx.send(Some((bf, PathBuf::from(file_name))));
        }
        Err(e) => {
            println!("{}", e);
            let _ = tx.send(None);
        }
    }
    ctx.request_repaint();
}
