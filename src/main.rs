#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fmt;
use std::io::{self, Write};
use std::ptr::null;
use std::time::Duration;

use ansi_codes::{
    ansi::{hide_cursor, set_cursor_position, show_cursor},
    clear,
};

mod grid;
use grid::Grid;

const X: &str = "X";

fn main() {
    // clear();
    let starting_state = r#"
    X X X
    X X X
    X X X
    "#;
    let grid = init(starting_state);
    print!("{}", grid);
}

fn init(starting_state: &str) -> Grid{
    let mut grid: Grid;
    let mut array_size: usize = 0;
    for line in starting_state.lines(){
        array_size = line.matches(X).count();
        if array_size == 0{
            continue;
        }
        break
    }
    
    match Grid::new(&array_size) {
        Ok(g) => grid = g,
        Err(error) => panic!("{}", error)
    }

    match grid.set_input(starting_state){
        Ok(_) =>  {}
        Err(error) => {
            panic!("Error is: {}", error)
        }
    }
    // DON'T pass a reference
    // this is MOVING ownership from init() to the parent function
    return grid
}

fn progress_life(grid: &mut Grid) {}