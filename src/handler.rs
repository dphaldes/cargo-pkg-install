use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Ok, Result};

use crate::meta::Manifest;

pub fn handle_manifest(prefix: &str, manifest: Manifest) -> Result<()> {
    let prefix = Path::new(prefix);

    let bin_prefix = prefix.join("bin");
    std::fs::create_dir_all(&bin_prefix)?;

    let base_path = Path::new("target/release/");
    for bin_name in &manifest.meta.bin {
        let bin_path = base_path.join(bin_name);
        if !bin_path.is_file() {
            bail!("Binary Not Found: {}", bin_name);
        }

        let target_path = bin_prefix.join(bin_name);
        copy_file(bin_path, target_path)?;
    }

    Ok(())
}

fn copy_file(source: PathBuf, destination: PathBuf) -> Result<()> {
    std::fs::copy(&source, &destination).with_context(|| {
        format!(
            "Couldn't copy {} to {}",
            source.to_string_lossy(),
            destination.to_string_lossy()
        )
    })?;

    Ok(())
}
