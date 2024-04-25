use std::fmt;
use std::ptr::null;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

use ansi_codes::{
    ansi::{hide_cursor, set_cursor_position, show_cursor},
    clear,
};

// Define a constant for the array size
const ARRAY_SIZE: usize = 3;

// Define a custom struct to hold the array
#[derive(Default)]
struct Grid {
    // data: [[bool; ARRAY_SIZE]; ARRAY_SIZE],
    data: Vec<Vec<bool>>,
    // data_ptrs: [*const bool; ARRAY_SIZE*ARRAY_SIZE],
    data_ptrs: Vec<*const bool>
}



impl Grid {
    fn new( n: u8) -> Self {
        const CUSTOM_ARRAY_SIZE: u8 = n;
        match n {
            Some(u8) => {
                let data= vec![];
                let data_ptrs: [*const bool; n*n];
                let s = &Self{
                    data: data,
                    data_ptrs: data_ptrs
                };
                let i = 0;
                for row in data{
                    for cell in row{
                        data_ptrs[0] = &cell;
                        i += 1;
                    }
                }
                return s
            }
            None => {}
        }
    }

    fn foo(){}
}

// Implement Display trait for Grid
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for item in row {
                let out = if *item { 1 } else { 0 };
                write!(f, "{}", out)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



fn main() {
    clear();
    hide_cursor();
    // show_cursor();

    let mut grid: Grid = Default::default();
    let mut i: u8 = 0;
    println!("{}", grid);

    loop{
        progress_life(&mut grid);
        i += 1;
        println!("{:?}", i);
        set_cursor_position(1, 1);
        print!("x");
        set_cursor_position(0, 6);
        print!("END");
        println!("");
        println!("");
        println!("");
        thread::sleep(Duration::from_secs(1));
    }
}

fn progress_life(grid: &mut Grid) {

}