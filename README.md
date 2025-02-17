# cargo-pkg-install

Distribute additional files (documentations, desktop files) along with your rust application.

## Usage
Install this program and then add  `[meta]` to your Cargo.toml file 

```toml
[meta]
bin = ["app-name"]  # executable names matching Cargo.toml
```

Run `cargo pkg-install` in the project root

