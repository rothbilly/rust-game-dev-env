use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub enum SerializeMode {
    JSON,
    TOML,
    BIN,
}


// Serializable Interface
pub trait ISerializableData: Serialize + Default {
    fn save_with_mode<P: AsRef<Path>>(
        &self,
        mode: SerializeMode,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match mode {
            SerializeMode::JSON => self.save_as_json(path)?,
            SerializeMode::BIN => self.save_as_bin(path)?,
            SerializeMode::TOML => self.save_as_toml(path)?,
        };
        Ok(())
    }

    fn save_as_json<P: AsRef<Path>>(&self, _path: P) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = _path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        };
        let json = serde_json::to_string_pretty(self)?;
        fs::write(_path.as_ref(), json)?;
        Ok(())
    }

    /// Save settings to a custom binary file path (accepts `String`, `&str`, or `&Path`).
    fn save_as_bin<P: AsRef<Path>>(&self, _path: P)  -> Result<(), Box<dyn std::error::Error>>{
        if let Some(parent) = _path.as_ref().parent(){
            fs::create_dir_all(parent)?  
        };
        
        let bytes = bincode::serialize(self)?;
        fs::write(_path.as_ref(), bytes)?;
        Ok(())
    }

    /// Save settings to a custom TOML file path (accepts `String`, `&str`, or `&Path`).
    fn save_as_toml<P: AsRef<Path>>(&self, _path: P)  -> Result<(), Box<dyn std::error::Error>>{
        if let Some(parent) = _path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        };
        
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(_path.as_ref(), toml_str)?;
        Ok(())
    }


    // fn load_with_mode<T>(mode: SerializeMode, path: &Path) -> Result<T, Box<dyn std::error::Error>>
    // where
    //     T: for<'de> Deserialize<'de>;
    // {

    // }

    /// Load settings from a custom binary file path (accepts `String`, `&str`, or `&Path`).
    fn load_from_bin_path<T: ISerializableData>(_mode: SerializeMode, _path: &Path) -> Result<T, Box<dyn std::error::Error>> 
    where 
        T: for<'de> Deserialize<'de>
    {
        if let Ok(bytes) = fs::read(_path)
            && let Ok(des) = bincode::deserialize::<T>(&bytes){
                return Ok(des)
            }
        Ok(T::default())
    }

}
