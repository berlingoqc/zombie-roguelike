use crate::generation::position::Position;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub position: Position,
    pub level_iid: String,
    pub size: (i32, i32),
}
