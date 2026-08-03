
use std::{hash::Hash, path::PathBuf};
use macroquad::{audio::Sound, prelude::*};



pub enum ResourceDataType {
    Texture(Texture2D),
    Audio(Sound),
    Mesh(Mesh),
    Unknown,
}


pub struct AssetWrapper {
    pub data_type: ResourceDataType,
    pub data_bytes: Vec<u8>,
}


pub struct Resource {
    pub path: PathBuf,
    pub uuid: u128,
    pub asset: AssetWrapper,
}

impl Hash for Resource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
        self.path.hash(state);
    }
}