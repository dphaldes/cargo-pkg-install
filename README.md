# cargo-pkg-install

Distribute additional files (documentations, desktop files) along with your rust application.

## Usage
Install this program and then add  `Package.toml` to your project root 

```toml
[package]
bin = ["app-name"]  # executable names matching Cargo.toml
```

Run `cargo pkg-install`

