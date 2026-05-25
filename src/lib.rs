
/// Responsible for the lifecycle of objects, rendering, scenes, in-game UI, audio, and physics.
pub mod core {
    pub mod app;
    pub mod game;
    pub mod ui;

    pub mod rendering {
        pub mod letterbox;
    }
}

/// Data model, resources, loading unloading assets, serialization and managing files.
pub mod data {
    pub mod serialization;
    pub mod inspectable;
    pub mod settings;
    pub mod assets;
}

/// Virtual Games 
pub mod mini_games {
    pub mod main_game;
    pub mod macrowareexe;
}

/// App Inspectors & Debuggers
pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}



/// Helpers, Macros and external toolings
pub mod toolbox {
    pub mod mesh_tools;
    pub mod macros;
    pub mod file_reader;
    
}