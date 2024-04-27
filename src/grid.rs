use std::fmt;

pub struct Grid {
    arr: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(n: &usize) -> Self {
        let arr: Vec<Vec<bool>> = vec![vec![false; *n]; *n];
        return Self { arr };
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.arr{
            for cell in row{
                write!(f, "{}",  if *cell {1} else {0});
                write!(f, " ");
            }
            writeln!(f, " ");
        }
        Ok(())
    }
}
