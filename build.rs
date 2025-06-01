fn main() -> std::io::Result<()> {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_icon("assets/images/tetris.ico")
            .compile()?;
    }
    Ok(())
}
