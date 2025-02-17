mod handler;
mod meta;

use std::path::Path;

use anyhow::{bail, Context, Ok, Result};
use meta::Manifest;

fn main() -> Result<()> {
    // let prefix = "~/.local";
    let prefix = "./local/";

    let path = Path::new("Cargo.toml");
    if !path.is_file() {
        bail!("Error: Cargo.toml not found. Run this command in rust project root");
    }

    let contents = std::fs::read_to_string(path).with_context(|| "Couldn't read Cargo.toml")?;
    let data: Manifest = toml::from_str(&contents)?;

    handler::handle_manifest(prefix, data)?;

    Ok(())
}
