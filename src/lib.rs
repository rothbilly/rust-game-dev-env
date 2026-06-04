
pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}

pub mod toolbox {
    pub mod mesh_tools;
    pub mod utils {
    }
}

pub mod data {
    pub mod serialization;
    pub mod settings;
}

pub mod core {
    pub mod app;
    pub mod game;
    pub mod rendering {
        pub mod letterbox;
        pub mod shader;
    }
}


pub mod components {
    pub mod gameobject;
}

pub mod games {
    pub mod breakout;
    pub mod macrowareexe;
    pub mod empty_game;


    pub use breakout::TestGame;
    pub use macrowareexe::MacrowareExe;
}
