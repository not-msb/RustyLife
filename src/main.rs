mod patterns;

use std::{fs::{File, self}, io::Write};
use patterns::*;

const WIDTH:  usize = 100;
const HEIGHT: usize = 100;

type Field = [[u8; WIDTH]; HEIGHT];

fn main() {
    let folder = "video";
    if fs::read_dir(folder).is_ok() {
        fs::remove_dir(folder).unwrap()
    }
    fs::create_dir(folder).unwrap();

    let mut field: Field = [[0; WIDTH]; HEIGHT];

    field = glider_generator(field);

    for i in 0..10000 {
        let path: &str = &format!("video/cgol-{}.ppm", i);
        if File::open(path).is_ok() {
            fs::remove_file(path).unwrap();
        }
        let mut file = File::create(path).unwrap();
        if i == 0 {
            write_field_to_ppm(&mut file, field);
            continue;
        };

        field = calculate_changes(field);
        write_field_to_ppm(&mut file, field);
        println!("rendered file {}", i);
    }
}

fn calculate_changes(field: Field) -> Field {
    let mut new_field: Field = [[0; WIDTH]; HEIGHT];
    for h in 0..HEIGHT as isize {
        for w in 0..WIDTH as isize {
            let ohl = if h == 0 { -(HEIGHT as isize-1)} else { 1 };
            let owl = if w == 0 { -(WIDTH as isize-1)} else { 1 };
            let ohh = if h == HEIGHT as isize-1 { -(HEIGHT as isize-1) } else { 1 };
            let owh = if w == WIDTH as isize-1 { -(WIDTH as isize-1) } else { 1 };
            let neighbours =
                field[(h-ohl) as usize][(w-owl) as usize] + field[(h-ohl) as usize][w as usize] + field[(h-ohl) as usize][(w+owh) as usize] +
                field[h as usize][(w-owl) as usize]       +                                       field[h as usize][(w+owh) as usize]       +
                field[(h+ohh) as usize][(w-owl) as usize] + field[(h+ohh) as usize][w as usize] + field[(h+ohh) as usize][(w+owh) as usize];
            if field[h as usize][w as usize] == 0 {
                if neighbours == 3 {
                    new_field[h as usize][w as usize] = 1;
                }
            } else if field[h as usize][w as usize] == 1 && (neighbours == 2 || neighbours == 3) {
                new_field[h as usize][w as usize] = 1;
            }
        }
    }

    new_field
}

fn write_field_to_ppm(file: &mut File, field: Field) {
    let header = format!("P6 {} {} 255\n", WIDTH, HEIGHT);
    file.write_all(header.as_bytes()).unwrap();
    for h in field.iter() {
        for w in h.iter() {
            let pixel = &[
                255 - *w*255,
                255 - *w*255,
                255 - *w*255
                ];
            file.write_all(pixel).unwrap();
        }
    }
}