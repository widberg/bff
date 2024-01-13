use std::io::Error;

fn main() -> Result<(), Error> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/bff.ico");
        res.compile()?;
        println!("cargo:rerun-if-changed=resources/bff.ico");
    }
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
