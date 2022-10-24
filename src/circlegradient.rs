pub fn circle_gradient(size: i64) -> Vec<Vec<u8>> {
	let mut grid = vec![vec![0_f64; size as usize]; size as usize];
	for y in 0..size {
		for x in 0..size {

			let vecx = ( x as f64 + 0.5 ) - ( size as f64 / 2.0 );
			let vecy = ( y as f64 + 0.5 ) - ( size as f64 / 2.0 );

			let vec = ( ( vecx*vecx ) + ( vecy*vecy ) ).sqrt();
			grid[y as usize][x as usize] = vec;
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

    let mut final_grid = vec![vec![0_u8; 500]; 500];
    for y in 0..500 {
        for x in 0..500 {
            let value = grid[y][x];
            let scaled: u8 = ( ( (value - min) / (max - min) ) * 255.0 ) as u8;
            final_grid[y][x] = scaled;
        }
    }

    return final_grid
}