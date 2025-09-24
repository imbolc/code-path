use anyhow::{Context, Result};
use code_path::code_path;

fn main() -> anyhow::Result<()> {
    configure()
}

fn configure() -> Result<()> {
    let filename = "config.toml";
    let _ = load_config(filename)
        .context(filename)
        .context(code_path!())?;
    Ok(())
}

fn load_config(filename: &str) -> Result<String> {
    std::fs::read_to_string(filename).context(code_path!())
}
