#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use std::thread::sleep;
use std::{any, fmt};
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
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ X _ _ _ _
    _ _ _ X _ X _ _ _ _
    _ _ _ _ X X _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _ _ _
    "#;
    let mut grid: Grid = init(starting_state);
    progress_life(&mut grid);
}

fn init(starting_state: &str) -> Grid{
    let mut grid: Grid;
    let mut array_size: usize = 0;
    for line in starting_state.lines(){
        array_size = line.replace(" ", "").len();
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
    // DON'T pass a reference.
    // this is MOVING ownership from init() to the parent function
    return grid
}

fn progress_life(grid: &mut Grid) {
    clear();
    print!("{}", grid);
    loop{
        // sleep(Duration::from_millis(40));
        sleep(Duration::from_secs(1));
        let update_array: Vec<[usize; 3]> = grid.life();
        for update in update_array.iter(){
            // println!("{}, {}", update[0], update[1]);
            set_cursor_position(update[0], update[1]);
            print!("=")
        }
        break;
        clear();
        print!("{}", grid);
        if update_array.len() == 0{
            break;
        }
    }
}