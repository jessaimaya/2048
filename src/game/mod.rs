use log::info;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
struct GridPoint(u8, u8);

impl Distribution<GridPoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridPoint {
        let (rand_x, rand_y) = (rng.gen_range(0..4), rng.gen_range(0..4));
        GridPoint(rand_x, rand_y)
    }
}

pub fn add_random_2(grid: &mut [[u8; 4]; 4]) -> &mut [[u8; 4]; 4] {
    let mut rng = rand::thread_rng();
    let mut rand_grid_point: GridPoint = rng.gen();

    while grid[rand_grid_point.0 as usize][rand_grid_point.1 as usize] != 0 {
        rand_grid_point = rng.gen();
    }

    grid[rand_grid_point.0 as usize][rand_grid_point.1 as usize] = 2;
    grid
}
