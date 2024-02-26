
#[derive(Debug, Clone)]
pub struct Position(pub i32,pub i32);

impl std::fmt::Display for Position {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}x{})", self.0, self.1)
    }
    
}
