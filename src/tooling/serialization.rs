use egui_macroquad::egui::util::undoer::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub enum SerializeMode {
    JSON,
    TOML,
    BIN,
    RON,
}

trait SerializableData: serde::Serialize {
    fn save_with_mode(
        &self,
        mode: SerializeMode,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = match mode {
            SerializeMode::JSON => self.save_to_json(path)?,
            _ => todo!()
        };
        //fs::write(path, data)?;
        Ok(())
    }
    fn load<T>(mode: SerializeMode, path: &Path) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>;

    fn save_to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent)?;
        };
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path.as_ref(), json)?;
        Ok(())
    }
}
