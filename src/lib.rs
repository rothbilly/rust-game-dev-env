pub mod entities {}

pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui;
}

pub mod tools {
    pub mod mesh_tools;
    pub mod utils {
        pub mod serialization;
    }
}

pub mod data {
    pub mod serializables;
    pub mod settings;
}

pub mod core {
    pub mod app;
    pub mod game;
}

pub mod rendering {
    pub mod letterbox;
}

pub mod gameobjects {
    pub mod gameobject;
}

pub mod games {
    pub mod test_game;
    pub mod macrowareexe;

    pub use test_game::TestGame;
    pub use macrowareexe::MacrowareExe;
}

pub mod macros {
    pub mod rules;
}