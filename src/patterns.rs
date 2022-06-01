#![allow(dead_code)]

use crate::Field;
use super::WIDTH;

pub fn glider(mut field: Field) -> Field {
    field[5 + 5 * WIDTH as isize] = 1;
    field[6 + 6 * WIDTH as isize] = 1;
    field[7 + 4 * WIDTH as isize] = 1;
    field[7 + 5 * WIDTH as isize] = 1;
    field[7 + 6 * WIDTH as isize] = 1;

    field
}

pub fn glider_generator(mut field: Field) -> Field {
    field[5 + 5 * WIDTH as isize] = 1;
    field[5 + 6 * WIDTH as isize] = 1;
    field[6 + 5 * WIDTH as isize] = 1;
    field[6 + 6 * WIDTH as isize] = 1;
    
    field[5 + 15 * WIDTH as isize] = 1;
    field[6 + 15 * WIDTH as isize] = 1;
    field[7 + 15 * WIDTH as isize] = 1;
    field[4 + 16 * WIDTH as isize] = 1;
    field[8 + 16 * WIDTH as isize] = 1;
    field[3 + 17 * WIDTH as isize] = 1;
    field[3 + 18 * WIDTH as isize] = 1;
    field[9 + 17 * WIDTH as isize] = 1;
    field[9 + 18 * WIDTH as isize] = 1;
    
    field[6 + 19 * WIDTH as isize] = 1;

    field[4 + 20 * WIDTH as isize] = 1;
    field[8 + 20 * WIDTH as isize] = 1;
    field[5 + 21 * WIDTH as isize] = 1;
    field[6 + 21 * WIDTH as isize] = 1;
    field[7 + 21 * WIDTH as isize] = 1;
    field[6 + 22 * WIDTH as isize] = 1;

    field[3 + 25 * WIDTH as isize] = 1;
    field[4 + 25 * WIDTH as isize] = 1;
    field[5 + 25 * WIDTH as isize] = 1;
    field[3 + 26 * WIDTH as isize] = 1;
    field[4 + 26 * WIDTH as isize] = 1;
    field[5 + 26 * WIDTH as isize] = 1;
    field[2 + 27 * WIDTH as isize] = 1;
    field[6 + 27 * WIDTH as isize] = 1;

    field[1 + 29 * WIDTH as isize] = 1;
    field[2 + 29 * WIDTH as isize] = 1;
    field[6 + 29 * WIDTH as isize] = 1;
    field[7 + 29 * WIDTH as isize] = 1;

    field[3 + 39 * WIDTH as isize] = 1;
    field[4 + 39 * WIDTH as isize] = 1;
    field[3 + 40 * WIDTH as isize] = 1;
    field[4 + 40 * WIDTH as isize] = 1;

    field
}