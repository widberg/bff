use std::io::Error;

fn main() -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    {
        // https://users.rust-lang.org/t/compile-for-windows-from-linux-when-have-build-rs/76858
        let target_arch = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

        if target_arch == "windows" {
            let mut res = winres::WindowsResource::new();
            res.set_icon("resources/bff.ico");
            res.compile()?;
        }
    }

    Ok(())
}
