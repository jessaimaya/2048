use log::info;
use mogwai::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Grid {
    pub data: [[u16; 4]; 4],
    pub score: u16,
    pub score_add: u16,
    pub highest: u16,
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

    pub fn move_up(&mut self) -> (u16, u16, u16) {
        self.transpose();
        self.move_left();
        self.transpose();
        (self.score, self.score_add, self.highest)
    }

    pub fn move_left(&mut self) -> (u16, u16, u16) {
        self.compress();
        self.merge();
        self.compress();
        (self.score, self.score_add, self.highest)
    }

    pub fn move_right(&mut self) -> (u16, u16, u16) {
        self.reverse();
        self.move_left();
        self.reverse();
        (self.score, self.score_add, self.highest)
    }
    pub fn move_down(&mut self) -> (u16, u16, u16) {
        self.transpose();
        self.move_right();
        self.transpose();
        (self.score, self.score_add, self.highest)
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
                    let add = self.data[i][j] * 2;
                    self.score_add = self.data[i][j];
                    self.data[i][j] = add;
                    self.score += add;
                    self.data[i][j + 1] = 0;

                    if self.data[i][j] > self.highest {
                        self.highest = self.data[i][j];
                    }
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
    pub fn base_grid_view(&self) -> ViewBuilder<HtmlElement> {
        let grid = self.data;
        builder! {
            <div class="tiles">
                { if grid[0][0] != 0 { render_card(grid[0][0], 0, 0)  } else{builder!{<span></span>}}}
               { if grid[0][1] != 0 { render_card(grid[0][1], 0, 1)  } else{builder!{<span></span>}}}
               { if grid[0][2] != 0 { render_card(grid[0][2], 0, 2)  } else{builder!{<span></span>}}}
               { if grid[0][3] != 0 { render_card(grid[0][3], 0, 3)  } else{builder!{<span></span>}}}

               { if grid[1][0] != 0 { render_card(grid[1][0], 1, 0)  } else{builder!{<span></span>}}}
               { if grid[1][1] != 0 { render_card(grid[1][1], 1, 1)  } else{builder!{<span></span>}}}
               { if grid[1][2] != 0 { render_card(grid[1][2], 1, 2)  } else{builder!{<span></span>}}}
               { if grid[1][3] != 0 { render_card(grid[1][3], 1, 3)  } else{builder!{<span></span>}}}

               { if grid[2][0] != 0 { render_card(grid[2][0], 2, 0)  } else{builder!{<span></span>}}}
               { if grid[2][1] != 0 { render_card(grid[2][1], 2, 1)  } else{builder!{<span></span>}}}
               { if grid[2][2] != 0 { render_card(grid[2][2], 2, 2)  } else{builder!{<span></span>}}}
               { if grid[2][3] != 0 { render_card(grid[2][3], 2, 3)  } else{builder!{<span></span>}}}

               { if grid[3][0] != 0 { render_card(grid[3][0], 3, 0)  } else{builder!{<span></span>}}}
               { if grid[3][1] != 0 { render_card(grid[3][1], 3, 1)  } else{builder!{<span></span>}}}
               { if grid[3][2] != 0 { render_card(grid[3][2], 3, 2)  } else{builder!{<span></span>}}}
               { if grid[3][3] != 0 { render_card(grid[3][3], 3, 3)  } else{builder!{<span></span>}}}

           </div>

        }
    }
    pub fn is_same(&self, other: Grid) -> bool {
        let mut is_same = true;
        for (ind, item) in self.data.iter().enumerate() {
            for (pos, elem) in item.iter().enumerate() {
                if other.data[ind][pos] != *elem {
                    is_same = false;
                    break;
                }
            }
        }

        is_same
    }
    pub fn is_full(&self) -> bool {
        let mut is_full = true;
        for item in self.data.iter() {
            for elem in item.iter() {
                if *elem == 0 {
                    is_full = false;
                    break;
                }
            }
        }
        info!("is full?: {:?}", self.data);
        is_full
    }
}

pub fn render_card(value: u16, col: u16, row: u16) -> ViewBuilder<HtmlElement> {
    builder! {
        <div
            class={
                format!(
                    "card card__{val} position__{c}_{r} {small}",
                    val = value.to_string(),
                    c = col,
                    r = row,
                    small = if value > 64 { "small" } else { "" }
                )
            }
        >
            {value.to_string()}
        </div>
    }
}

pub fn render_board() -> ViewBuilder<HtmlElement> {
    builder! {
        <div class="board">
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
               <div class="tile"></div>
        </div>
    }
}

#[derive(Debug)]
struct GridPoint(u16, u16);

impl Distribution<GridPoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridPoint {
        let (rand_x, rand_y) = (rng.gen_range(0..4), rng.gen_range(0..4));
        GridPoint(rand_x, rand_y)
    }
}
