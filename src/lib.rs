// make macros public to all mods in this crate.

// ==========================================
//  Engine mods
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
    pub mod engine;
    pub mod app;
    pub mod game;
    pub mod rendering {
        pub mod letterbox;
        pub mod post_processing;
        pub mod shader;
    }
    pub mod math;
    pub mod signal;
}

pub mod components {
    pub mod gameobject;
}

// ==========================================
//  Games mods - in the future in its own crate maybe
// ==========================================

pub mod games {
    // pub mod breakout;
    // pub mod macrowareexe;
    // ...
    automod::dir!(pub "src/games");
    
}
