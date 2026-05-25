
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
    pub mod inspectable;
    pub mod settings;
}

/// App Inspectors & Debuggers
pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}

/// Virtual Games 
pub mod mini_games {
    pub mod test_game;
    pub mod macrowareexe;
    pub use test_game::TestGame;
    pub use macrowareexe::MacrowareExe;
}

/// Helpers, Macros and external toolings
pub mod tooling {
    pub mod mesh_tools;
    pub mod serialization;
    pub mod macros;
    
}