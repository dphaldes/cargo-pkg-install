use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub bin: Vec<String>,
}
