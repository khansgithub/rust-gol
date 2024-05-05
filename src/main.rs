#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fmt;
use std::io::{self, Write};
use std::ptr::null;
use std::thread;
use std::time::Duration;

use ansi_codes::{
    ansi::{hide_cursor, set_cursor_position, show_cursor},
    clear,
};

mod grid;
use grid::Grid;

// Define a constant for the array size
const ARRAY_SIZE: usize = 5;

fn main() {
    clear();
    let mut grid: Grid = Grid::new(&ARRAY_SIZE);
    let starting_state = r#"
    X X X
    X X X
    X X X
    "#;
    grid.set_input(starting_state.clone());
    print!("{}", grid);
}

fn progress_life(grid: &mut Grid) {}