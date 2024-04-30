use std::fmt;

#[derive(Debug)]
pub struct Grid {
    arr: Vec<Vec<bool>>,
    coordinates: Vec<[u8;2]>
}

impl Grid {
    pub fn new(n: &usize) -> Self {
        let mut arr: Vec<Vec<bool>> = vec![vec![false; *n]; *n];
        let mut coordinates: Vec<[u8; 2]> = Vec::with_capacity(n*n);

        let max: u8 = *n as u8;
        for y in 0..max{
            for x in 0..max{
                let data: [u8; 2] = [x, y];
                coordinates.push(data)
            }
        }

        println!("{:?}", coordinates);
        return Self { arr, coordinates };
    }

    pub fn arr(&self) -> & Vec<Vec<bool>> {
        &self.arr
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = (-1, -1);
        for row in &self.arr{
            for cell in row{
                write!(f, "{}",  if *cell {1} else {0})?;
                write!(f, " ")?;
            }
            writeln!(f, " ")?;
        }
        Ok(())
    }
}
