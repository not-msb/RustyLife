mod patterns;

use patterns::*;
use std::{
    fs::{self, File},
    io::{Write, BufWriter},
    ops::{Index, IndexMut}, time::Instant,
};

const WIDTH:        usize = 100;
const HEIGHT:       usize = 100;
const LENGTH:       usize = WIDTH * HEIGHT;
const PPM_SCALER:   usize = 10;
const VIDEO_LENGTH: usize = 1000;

#[derive(Debug, Clone, Copy)]
pub struct Field {
    data: [u8; LENGTH],
}

impl Field {
    pub fn new() -> Self {
        Field { data: [0; LENGTH] }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field::new()
    }
}

impl Index<isize> for Field {
    type Output = u8;

    fn index(&self, i: isize) -> &u8 {
        let index = (i+LENGTH as isize)%LENGTH as isize;
        &self.data[index as usize]
    }
}

impl IndexMut<isize> for Field {
    fn index_mut(&mut self, i: isize) -> &mut u8 {
        let index = (i+LENGTH as isize)%LENGTH as isize;
        &mut self.data[index as usize]
    }
}

fn main() {
    let now = Instant::now();

    let folder = "video";
    if fs::read_dir(folder).is_ok() {
        fs::remove_dir(folder).unwrap()
    }
    fs::create_dir(folder).unwrap();

    let mut field: Field = Field::new();

    // field = glider(field);
    field = glider_generator(field);

    for i in 0..VIDEO_LENGTH {
        let path: &str = &format!("video/cgol-{}.ppm", i);
        let mut file = File::create(path).unwrap();
        let writer = BufWriter::new(&mut file);
        field = calculate_changes(&field);
        write_field_to_ppm(writer, field);
        println!("rendered file {}", i);
    }

    println!("{:?}", now.elapsed());
}

pub fn calculate_changes(field: &Field) -> Field {
    let mut new_field: Field = Field::new();
    for i in 0..LENGTH as isize {
        let mut neighbours = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                if y==0 && x==0 { continue }
                neighbours += field[i+y*WIDTH as isize+x];
            }
        }

        new_field[i] = ((field[i] == 0 && neighbours == 3) || (field[i] == 1 && (neighbours == 2 || neighbours == 3))) as u8;
    }

    new_field
}

pub fn write_field_to_ppm(mut file: BufWriter<&mut File>, field: Field) {
    let header = format!("P6 {} {} 255\n", WIDTH*PPM_SCALER, HEIGHT*PPM_SCALER);
    file.write_all(header.as_bytes()).unwrap();
    for h in 0..HEIGHT*PPM_SCALER {
        for w in 0..WIDTH*PPM_SCALER {
            let i = w/PPM_SCALER + h/PPM_SCALER*WIDTH;
            let c = 255-field[i as isize]*255;
            file.write_all(&[c; 3]).unwrap();
        }
    }
}
