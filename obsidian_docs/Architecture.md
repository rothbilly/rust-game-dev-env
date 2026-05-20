
back to [README](README.md)

### Standart rust proyect architecture

```
my_game/
├── Cargo.toml
├── assets/                  # Shaders, GLTF models, textures, audio
│   └── models/
│       └── character.glb
├── src/
│   ├── main.rs              # Thin wrapper: sets up engine & boots the game loop
│   ├── lib.rs               # Houses the core game library
│   ├── assets.rs            # Handles loading/caching of assets (GLTF, textures)
│   ├── components.rs        # Pure data structs (Position, Health, Input)
│   ├── systems/             # Pure logic modules
│   │   ├── mod.rs
│   │   ├── physics.rs       # Movement, collisions
│   │   └── rendering.rs     # Taking data and drawing it to the screen
│   ├── game_state.rs        # State machines (MainMenu, Playing, Paused)
│   └── ui.rs                # Heads-up display, menus
```


## lib.rs

```rust
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
    pub mod macros {
	    pub mod rules;
    }
}

pub mod data {
    pub mod serializables;
    pub mod settings;
}

pub mod core {
    pub mod app{
    }
    pub mod game;
    
    pub mod mini_games {
	    pub mod test_game;
	}
}
pub mod rendering {
    pub mod letterbox;
}
pub mod games {
    pub mod test_game;
}


```