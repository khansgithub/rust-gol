use std::fmt;

#[derive(Default)]
pub struct Grid{}

impl Grid {
    pub fn new() -> Self {
        return Self{};
    }

    fn default() -> Self{
        return Self{};
    }
}


impl fmt::Display for Grid{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "test")
    }
}