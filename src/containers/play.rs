use log::info;
use mogwai::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

use crate::components::grid::*;
use crate::router::Route;

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
    pub win: bool,
    pub score: u32,
    pub score_add: u16,
    pub highest: u16,
}

#[derive(Clone, Debug)]
pub enum PlayModelIn {
    KeyUp(Option<Event>),
    Restart,
    Undo,
}

#[derive(Clone)]
pub enum PlayViewOut {
    Moved(Option<Move>, Grid, u32, u16, u16),
    Win,
    GameOver,
}

impl Default for Play {
    fn default() -> Self {
        let mut grid: Grid = Grid {
            data: [[0; 4]; 4],
            score: 0,
            score_add: 0,
            highest: 0,
        };
        grid.add_random_2();

        Play {
            grid: vec![grid],
            last_move: None,
            game_over: false,
            win: false,
            score: 0,
            score_add: 0,
            highest: 0,
        }
    }
}

impl Component for Play {
    type ModelMsg = PlayModelIn;
    type ViewMsg = PlayViewOut;
    type DomNode = HtmlElement;

    fn bind(&self, _input_sub: &Subscriber<PlayModelIn>) {
        window().focus().unwrap();
    }

    fn update(
        &mut self,
        msg: &PlayModelIn,
        tx: &Transmitter<PlayViewOut>,
        _sub: &Subscriber<PlayModelIn>,
    ) {
        window().focus().unwrap();
        if !self.game_over {
            match msg {
                PlayModelIn::KeyUp(evt) => {
                    let key = evt
                        .as_ref()
                        .expect("No keyboard event")
                        .unchecked_ref::<KeyboardEvent>()
                        .key();
                    let mut last: Grid = *self.grid.last().expect("Grid is empty");
                    let mut is_direction = true;

                    match key.as_ref() {
                        "ArrowUp" => {
                            self.last_move = Some(Move::Up);
                            let mv = last.move_up();
                            self.score += mv.0 as u32;
                            self.score_add = mv.1;
                            self.highest = mv.2;
                            info!("mv_score:{}, res_score:{}", mv.0, self.score);
                        }
                        "ArrowDown" => {
                            self.last_move = Some(Move::Down);
                            let mv = last.move_down();
                            self.score += mv.0 as u32;
                            self.score_add = mv.1;
                            self.highest = mv.2;
                            info!("mv_score:{}, res_score:{}", mv.0, self.score);
                        }
                        "ArrowLeft" => {
                            self.last_move = Some(Move::Left);
                            let mv = last.move_left();
                            self.score += mv.0 as u32;
                            self.score_add = mv.1;
                            self.highest = mv.2;
                            info!("mv_score:{}, res_score:{}", mv.0, self.score);
                        }
                        "ArrowRight" => {
                            self.last_move = Some(Move::Right);
                            let mv = last.move_right();
                            self.score += mv.0 as u32;
                            self.score_add = mv.1;
                            self.highest = mv.2;
                            info!("mv_score:{}, res_score:{}", mv.0, self.score);
                        }
                        _ => is_direction = false,
                    }

                    if is_direction {
                        let prev = *self.grid.last().expect("Grid is empty");
                        if !prev.is_same(last) {
                            // Check if we can move
                            last.add_random_2();
                            self.grid.push(last);
                            tx.send(&PlayViewOut::Moved(
                                self.last_move,
                                last,
                                self.score,
                                self.score_add,
                                self.highest,
                            ));
                        } else if last.is_full() {
                            self.game_over = true;
                            info!("GAME OVER");
                        }
                    }

                    if self.highest >= 2048 {
                        info!("WIN!");
                        // Win
                        self.win = true;
                        // tx.send(&PlayViewOut::Moved(self.last_move, last, self.score, self.score_add, self.highest));
                        tx.send(&PlayViewOut::Win)
                    }
                }
                PlayModelIn::Restart => {
                    let new = Play::default();
                    self.grid = new.grid;
                    self.last_move = new.last_move;
                    self.game_over = new.game_over;
                    tx.send(&PlayViewOut::Moved(
                        self.last_move,
                        self.grid.last().expect("Default grid is empty").to_owned(),
                        0,
                        0,
                        0,
                    ))
                }
                &PlayModelIn::Undo => {
                    if self.grid.len() > 1 {
                        self.grid.pop().expect("Couldn't undo.");
                        self.last_move = None;
                        self.score -= self.score_add as u32;
                        tx.send(&PlayViewOut::Moved(
                            self.last_move,
                            self.grid
                                .last()
                                .expect("Couldn't complete undo.")
                                .to_owned(),
                            self.score,
                            0,
                            self.highest,
                        ))
                    }
                }
                _ => (),
            }
        }
    }

    fn view(
        &self,
        tx: &Transmitter<PlayModelIn>,
        rx: &Receiver<PlayViewOut>,
    ) -> ViewBuilder<HtmlElement> {
        let loc_score = self.score;
        {
            builder!(
                <div
                    class="App"
                    tabindex="0"
                    on:keyup=tx.contra_map(|ev: &Event| PlayModelIn::KeyUp(Some(ev.clone())))
                >
                <a class="button green back_home" href="#/">
                        <span class="circle">"←"</span>
                </a>
                <div class="play__top">
                    <h2>"Score"</h2>
                    <h3>{("", rx.branch_map(|msg| {
                        match msg {
                        PlayViewOut::Win => format!("WIN"),
                        _ => format!("")
                    }}))}</h3>
                    <p class="score">
                        {(
                            "0",
                            rx.branch_map(move |msg| {
                                match msg {
                                    PlayViewOut::Moved(_mov, _grid, score_add, score, _highest) => {
                                        format!("{}", score_add.to_string())
                                     },
                                     _ => format!("{}", loc_score.to_string())
                                }
                             })
                        )}
                    </p>
                </div>
                <main class="wrapper"
                    patch:children=rx.branch_map(move |msg|{
                        if let PlayViewOut::Moved(_mov, grid, _score, _score_add, _highest) = msg {
                        Patch::Replace{value: grid.base_grid_view(), index: 1}.clone()
                    } else {
                        window().location().set_hash(&format!("#/win?sc={}", loc_score)).unwrap();
                        Patch::RemoveAll
                    }
                    }
                    )
                >
                    {render_board()}
                    {self.grid.last().expect("App grid empty").base_grid_view()}
                </main>
                <div class="play__bottom">
                    <a
                        class="button green play undo"
                        title="undo"
                        on:click = tx.contra_map(|_| PlayModelIn::Undo)
                    >
                        <span class="circle">"⭯"</span>
                    </a>
                    <a
                        title="restart"
                        class=" button green play"
                        on:click = tx.contra_map(|_| PlayModelIn::Restart)
                    >
                        "Restart"
                    </a>
                </div>
            </div>
            )
        }
    }
}
