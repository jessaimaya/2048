use log::info;
use mogwai::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

use crate::router::Route;
use crate::components::grid::*;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
pub struct Play {
    pub grid: Vec<Grid>,
    pub last_move: Option<Move>,
    pub game_over: bool,
}

#[derive(Clone)]
pub enum PlayModelIn {
    KeyUp(Option<Event>),
    _Restart,
    _Undo,
}

#[derive(Clone)]
pub enum PlayViewOut {
    Moved(Option<Move>, Grid),
}

impl Default for Play {
    fn default() -> Self {
        let mut grid: Grid = Grid { data: [[0;4]; 4]};
        grid.add_random_2();

        Play {
            grid: vec![grid],
            last_move: None,
            game_over: false,
        }
    }
}

impl Component for Play {
    type ModelMsg = PlayModelIn;
    type ViewMsg = PlayViewOut;
    type DomNode = HtmlElement;

    fn update( &mut self, msg: &PlayModelIn, tx: &Transmitter<PlayViewOut>, _sub: &Subscriber<PlayModelIn> ) {
        if !self.game_over {
            match msg {
                PlayModelIn::KeyUp(evt) => {
                    let key = evt.as_ref().expect("No keyboard event")
                        .unchecked_ref::<KeyboardEvent>()
                        .key();
                    let mut last: Grid = self.grid.last().expect("Grid is empty").clone();
                    let mut is_direction = true;

                    match key.as_ref() {
                        "ArrowUp" => {
                            self.last_move = Some(Move::Up);
                            last.move_up();
                        },
                        "ArrowDown" => {
                            self.last_move = Some(Move::Down);
                            last.move_down();
                        },"ArrowLeft" => {
                            self.last_move = Some(Move::Left);
                            last.move_left();
                        },"ArrowRight" => {
                            self.last_move = Some(Move::Right);
                            last.move_right();
                        },
                        _ => is_direction = false,
                    }
                    
                    if is_direction {
                        let prev = *self.grid.last().expect("Grid is empty");
                        if !prev.is_same(last) {
                            // Check if we can move
                            last.add_random_2();
                            self.grid.push(last);
                            tx.send(&PlayViewOut::Moved(self.last_move.clone(), last));
                        } else if last.is_full() {
                            self.game_over = true;
                            info!("GAME OVER");
                        }
                    }
                }
                Restart => (),
                Undo => (),
                _ => (),
            }
        }
    }

        fn view (&self, tx: &Transmitter<PlayModelIn>, rx: &Receiver<PlayViewOut>) -> ViewBuilder<HtmlElement> {
            builder!(
            <div
                class="App"
                tabindex="0"
                on:keyup=tx.contra_map(|ev: &Event| PlayModelIn::KeyUp(Some(ev.clone())))
            >
            <main class="wrapper"
                patch:children=rx.branch_map(|PlayViewOut::Moved(mov, grid)| {
                    Patch::Replace{value: grid.base_grid_view(), index: 1}.clone()
                })
            >
                {render_board()}
                {self.grid.last().expect("App grid empty").base_grid_view()}
            </main>
            </div>
        )
    }
}

