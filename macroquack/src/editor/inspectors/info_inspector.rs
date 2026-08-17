use engine_macros::{IName, IVisibility};

#[derive(Default, IName, IVisibility)]
pub struct GameInfoInspector {
    is_visible: bool,
    name: String,
}
