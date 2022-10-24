extern crate noise;
pub mod circlegradient;

use circlegradient::circle_gradient;
use std::{error::Error, fs, io, io::Write};
use noise::{MultiFractal, utils::*, Perlin, Fbm};

fn ndarray_to_csv(ndarray: Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let mut rows_as_str: Vec<String> = vec![];
    for y in ndarray.iter() {
        let vec_str: Vec<String> = y.iter().map( |&x| x.to_string() ).collect();     
        let mut joinedstr: String = vec_str.join(",");
        joinedstr.push_str("\n");
        rows_as_str.push(joinedstr);
    }
    let joined_rows_as_str: String = rows_as_str.join("");
    fs::write("output.csv", joined_rows_as_str.as_str())?;
    Ok(())
}

fn main() {

    print!("\x1b[32m=>\x1b[0m Enter number for seed:\x1b[33m ");
    io::stdout().flush().expect("Err");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Err");
    input = input.trim().to_string();
    print!("\x1b[0m\n");
    io::stdout().flush().expect("Err");

    let raw_seed: u128 = input.trim().parse().unwrap();
    let seed: u32 =  (raw_seed % u32::MAX as u128) as u32;

    let mut grid = vec![vec![0_f64; 500]; 500];
    let fbm = Fbm::<Perlin>::new(seed).set_frequency(2.0);
    let noisemap = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(500, 500)
        .build();
    for y in 0..500 {
        for x in 0..500 {
            grid[y][x] = noisemap.get_value(x, y)
        }
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for row in grid.iter() {
        for val in row.iter() {
            if val < &min {
                min = *val;
            }
            if val > &max {
                max = *val;
            }
        }
    }

    let mut encoded = vec![vec![0_u8; 500]; 500];
    for y in 0..500 {
        for x in 0..500 {
            let value = grid[y][x];
            let scaled: u8 = ( ( (value - min) / (max - min) ) * 255.0 ) as u8;
            encoded[y][x] = scaled;
        }
    }

    let circle_mask = circle_gradient(500);

    let mut final_grid = vec![vec![0_8; 500]; 500];
    for y in 0..500 {
        for x in 0..500 {
            if encoded[y][x] > circle_mask[y][x] {
                final_grid[y][x] = encoded[y][x] - circle_mask[y][x];
            }
        }
    }

    match ndarray_to_csv(final_grid) {
        Ok(()) => (),
        Err(err) => println!("{:?}", err),
    }
}