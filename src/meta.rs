use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub meta: Metadata,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub bin: Vec<String>,
}
