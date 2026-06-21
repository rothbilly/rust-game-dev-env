

// make macros public to all mods in this crate.
pub (crate) use engine_macros::*;

// ==========================================
//  Engine
// ==========================================


pub mod editor {
    pub mod constants;
    pub mod inspector;
    pub mod ui_kit;
}

pub mod toolbox {
    pub mod mesh_tools;
    pub mod utils {
        pub mod macro_rules;
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
    pub mod signal;
    pub mod math;
}


pub mod components {
    pub mod gameobject;
}

// ==========================================
//  Games consumes the engine
// ==========================================


pub mod games {
    pub mod breakout;
    pub mod macrowareexe;
    pub mod empty_game;
    pub mod tutorial_game;
    pub mod virtual_app;

    pub use breakout::TestGame;
    pub use macrowareexe::MacrowareExe;
}
