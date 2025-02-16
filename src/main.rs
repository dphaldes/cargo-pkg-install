mod handler;
mod package;

use std::path::Path;

use anyhow::{bail, Context, Ok, Result};
use package::Manifest;

fn main() -> Result<()> {
    // let prefix = "~/.local";
    let prefix = "./local/";

    let path = Path::new("Package.toml");
    if !path.is_file() {
        bail!("Error: Package.toml not found");
    }

    let contents = std::fs::read_to_string(path).with_context(|| "Couldn't read Package.toml")?;
    let data: Manifest = toml::from_str(&contents)?;

    handler::handle_manifest(prefix, data)?;

    Ok(())
}
