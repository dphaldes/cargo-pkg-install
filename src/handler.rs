use std::{fs, path::Path};

use anyhow::{bail, Context, Result};

use crate::package::Manifest;

pub fn handle_manifest(prefix: &str, manifest: Manifest) -> Result<()> {
    let prefix = Path::new(prefix);

    let bin_prefix = prefix.join("bin");
    fs::create_dir_all(&bin_prefix)?;

    let base_path = Path::new("target/release/");
    for bin_name in &manifest.package.bin {
        let bin_path = base_path.join(bin_name);
        if !bin_path.is_file() {
            bail!("Binary Not Found: {}", bin_name);
        }

        let path = bin_prefix.join(bin_name);
        std::fs::copy(&bin_path, &path).with_context(|| {
            format!(
                "Couldn't copy {} to {}",
                bin_path.to_string_lossy(),
                path.to_string_lossy()
            )
        })?;
    }

    Ok(())
}
