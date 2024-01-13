use derive_more::From;

#[derive(Debug, From)]
enum BffGuiError {
    Io(std::io::Error),
    Shadow(shadow_rs::ShadowError),
}

type BffGuiResult<T> = Result<T, BffGuiError>;

fn main() -> BffGuiResult<()> {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/bff.ico");
        res.compile()?;
    }

    shadow_rs::new()?;

    Ok(())
}