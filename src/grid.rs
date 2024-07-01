use std::{fmt, fs::copy, iter};

use crate::X;

#[derive(Debug)]
pub struct Grid {
    pub arr: Vec<Vec<bool>>,
    // update_arr: *mut Vec<[usize; 3]>,
    size: usize,
    // coordinates: Vec<&_>
}

impl Grid {
    pub fn new(n: &usize) -> Result<Self, String> {
        if *n < 1 {
            return Err(format!("Input size is malformed: n = {}", n));
        }

        // set up a 2d vec filled wtih `false`
        let mut arr: Vec<Vec<bool>> = vec![vec![false; *n]; *n];
        // let mut update_arr: Vec<[usize; 3]> = Vec::new();
        // let mut coordinates: Vec<&_> = Vec::with_capacity(n*n);

        //
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
        // return Ok(Self { arr, update_arr, size: *n });
        return Ok(Self { arr, size: *n });
    }

    pub fn set_input(&mut self, input_str: &str) -> Result<(), String> {
        // by gpt. i haven't actually looked at closures properly yet
        let input_str: String = input_str.chars().filter(|&c| !c.is_whitespace()).collect();

        // if input_str.len() != self.size * self.size {
        //     return Err(format!(
        //         "Starting state values must conatins {} values",
        //         self.size * self.size
        //     ));
        // }

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

    pub fn life(&mut self) -> Vec<[usize; 3]>{
        let mut update_array: Vec<[usize; 3]> = Vec::new();
        let mut x: usize;
        let mut y: usize = 0;
        // let arr = &self.arr;
        for row in self.arr.iter() {
            x = 0;
            for c in row {
                // let cell = *c;
                // println!("Checking x: {}, y: {}", x, y);
                let neighbours: u8 = self.alive_neighbours(x, y);
                // println!("Alive neighbours: {}", neighbours);
                let new_cell: bool = self.apply_rule(c, neighbours);
                if new_cell != *c {
                    update_array.push([x, y, if new_cell { 1 as usize } else { 0 as usize }]);
                }
                x += 1;
            }
            y += 1;
        }

        for update in update_array.iter(){
            let _x: usize = update[0];
            let _y: usize = update[1];
            let _cell: bool = update[2] == 1;
            self.arr[_y][_x] = _cell;
        }
        return update_array;
    }

    fn apply_rule(&self, cell: &bool, neighbours: u8) -> bool{
        match *cell{
            true => {
                return (neighbours == 2) || (neighbours == 3)
            }
            false => {
                return neighbours == 3
            }
        }
    }

    fn alive_neighbours(&self, x: usize, y: usize) -> u8 {
        // use signed ints to check for negative values
        let offset: [i8; 3] = [-1, 0, 1];
        let offsets = |v: usize| {
            let _v: i8 = v as i8;
            return offset.clone().map(|o: i8| o + _v);
        };

        let x_values: [i8; 3] = offsets(x);
        let y_values: [i8; 3] = offsets(y);

        let mut sum: u8 = 0;

        for _x in x_values {
            for _y in y_values {
                if (_x < 0 || // no negative coorindates
                   _y < 0 ||
                   _x as usize == self.size || // no overflow
                   _y as usize == self.size ||
                   _x as usize == x && _y as usize == y) { // skip the same coor as the current cell
                    continue;
                }
                sum += self.arr[_y as usize][_x as usize] as u8;
            }
        }

        return sum;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = (-1, -1);
        for row in &self.arr {
            for cell in row {
                write!(f, "{}", if *cell { 'X' } else { ' ' })?;
                // write!(f, " ")?;
            }
            writeln!(f, " ")?;
        }
        Ok(())
    }
}
