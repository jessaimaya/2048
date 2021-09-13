use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub data: [[u8; 4]; 4],
}

impl Grid {
    pub fn add_random_2(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rand_grid_point: GridPoint = rng.gen();

        while self.data[rand_grid_point.0 as usize][rand_grid_point.1 as usize] != 0 {
            rand_grid_point = rng.gen();
        }

        self.data[rand_grid_point.0 as usize][rand_grid_point.1 as usize] = 2;
    }

    pub fn move_up(&mut self) {
        self.transpose();
        self.move_left();
        self.transpose();
    }

    pub fn move_left(&mut self) {
        self.compress();
        self.merge();
        self.compress();
    }

    pub fn move_right(&mut self) {
        self.reverse();
        self.move_left();
        self.reverse();
    }
    pub fn move_down(&mut self) {
        self.transpose();
        self.move_right();
        self.transpose();
    }

    pub fn transpose(&mut self) {
        let mut out = [[0; 4]; 4];
        for col in 0..4 {
            for row in 0..4 {
                out[col][row] = self.data[row][col];
            }
        }
        self.data = out;
    }

    pub fn compress(&mut self) {
        let mut changed = false;
        let mut new_grid = [[0; 4]; 4];

        for i in 0..4 {
            let mut pos = 0;
            for j in 0..4 {
                if self.data[i][j] != 0 {
                    new_grid[i][pos] = self.data[i][j];
                    if j != pos {
                        changed = true;
                    }
                    pos += 1;
                }
            }
        }
        self.data = new_grid;
    }

    pub fn merge(&mut self) {
        let mut changed = false;

        for i in 0..4 {
            for j in 0..3 {
                if self.data[i][j] == self.data[i][j + 1] && self.data[i][j] != 0 {
                    self.data[i][j] *= 2;
                    self.data[i][j + 1] = 0;
                    changed = true;
                }
            }
        }
    }

    pub fn reverse(&mut self) {
        let mut new_grid = [[0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                new_grid[i][j] = self.data[i][3 - j];
            }
        }

        self.data = new_grid;
    }
}

#[derive(Debug)]
struct GridPoint(u8, u8);

impl Distribution<GridPoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridPoint {
        let (rand_x, rand_y) = (rng.gen_range(0..4), rng.gen_range(0..4));
        GridPoint(rand_x, rand_y)
    }
}
