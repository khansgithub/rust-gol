use std::{cell, collections::btree_map::Range, fmt};

#[derive(Debug)]
pub struct Grid {
    arr: Vec<Vec<bool>>,
    size: usize,
    // coordinates: Vec<&_>
}

impl Grid {
    pub fn new(n: &usize) -> Result<Self, String> {
        if *n < 1 {
            return Err(format!("Input size is malformed: n = {}", n));
        }
        let mut arr: Vec<Vec<bool>> = vec![vec![false; *n]; *n];
        // let mut coordinates: Vec<&_> = Vec::with_capacity(n*n);

        let max: u8 = *n as u8;
        for y in 0..max {
            for x in 0..max {
                // let data: [u8; 2] = [x, y];
                // let ptr: &_ = &arr[y][x];
                // coordinates.push(ptr);
            }
        }

        // println!("{:?}", coordinates);
        // return Self { arr, coordinates };
        return Ok(Self { arr, size: *n });
    }

    pub fn set_input(&mut self, input_str: &str) -> Result<(), String> {
        // by gpt. i haven't actually looked at closures properly yet
        let input_str: String = input_str.chars().filter(|&c| !c.is_whitespace()).collect();

        if input_str.len() != self.size * self.size {
            return Err(format!(
                "Starting state values must conatins {} values",
                self.size * self.size
            ));
        }

        let mut i: usize = 0;
        for row in &mut self.arr {
            for y in 0..self.size {
                match input_str.chars().nth(i) {
                    Some(cell_state) => {
                        row[y] = cell_state == 'X';
                    }
                    None => {}
                }
                i += 1;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = (-1, -1);
        for row in &self.arr {
            for cell in row {
                write!(f, "{}", if *cell { 1 } else { 0 })?;
                write!(f, " ")?;
            }
            writeln!(f, " ")?;
        }
        Ok(())
    }
}
