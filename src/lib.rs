
pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}

pub mod toolbox {
    pub mod mesh_tools;
    pub mod utils {
    }
    pub mod macros {
        pub mod rules;
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
    }
}


pub mod gameobjects {
    pub mod gameobject;
}

pub mod mini_games {
    pub mod test_game;
    pub mod macrowareexe;

    pub use test_game::TestGame;
    pub use macrowareexe::MacrowareExe;
}
