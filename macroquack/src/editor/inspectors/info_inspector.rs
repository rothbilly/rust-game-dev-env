use engine_macros::{IVisibility, IName};




#[derive(Default, IName, IVisibility)]
pub struct GameInfoInspector{
    is_visible: bool,
    name: String
    
}