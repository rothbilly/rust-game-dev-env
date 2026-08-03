// make macros public to all mods in this crate.

// ==========================================
//  Engine mods
// ==========================================


pub mod prelude{
    pub use engine_macros::register_app;
    pub use engine_macros::auto_deref;
    pub use macroquad;
    pub use macroquad::prelude::*;
}

pub mod editor {
    pub mod main_editor;
    pub mod constants;
    pub mod inspector;
    pub mod inspectors{
        // every inspector on its own mod.
        pub mod event_inspector;
        pub mod info_inspector;
        pub mod app_loader;
        pub mod settings_inspector;
    }
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
    pub mod generics{
        pub mod double_buffer;
        pub mod grid;
    }
}

pub mod core {
    pub mod engine;
    pub mod app;
    pub mod game;
    pub mod rendering {
        pub mod vector_graphics;
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

pub use inventory;
// ==========================================
//  Games mods - in the future in its own crate maybe
// ==========================================


