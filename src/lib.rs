pub mod core {
    pub mod app;
    pub mod game;
    pub mod ui;

    pub mod rendering {
        pub mod letterbox;
    }
}

pub mod data {
    pub mod inspectable;
    pub mod settings;
}

pub mod mini_games {
    pub mod test_game;
    pub mod macrowareexe;

    pub use test_game::TestGame;
    pub use macrowareexe::MacrowareExe;
}

pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}

pub mod tooling {
    pub mod mesh_tools;
    pub mod serialization;
    pub mod macros;
    
}