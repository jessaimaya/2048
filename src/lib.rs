use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

mod components;
mod game;

use components::*;

use crate::AppModelIn::KeyUp;
use game::Grid;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
struct App {
    pub grid: Vec<Grid>,
}

#[derive(Clone)]
enum AppModelIn {
    KeyUp(Option<Event>),
    _Restart,
    _Undo,
}

#[derive(Clone)]
enum AppViewOut {
    Moved(Grid),
}

impl Default for App {
    fn default() -> Self {
        let mut grid: Grid = Grid { data: [[0; 4]; 4] };
        grid.add_random_2();
        App { grid: vec![grid] }
    }
}

impl Component for App {
    type ModelMsg = AppModelIn;
    type ViewMsg = AppViewOut;
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &AppModelIn,
        tx: &Transmitter<AppViewOut>,
        _sub: &Subscriber<AppModelIn>,
    ) {
        match msg {
            KeyUp(evt) => {
                let key = evt
                    .as_ref()
                    .expect("No KeyboardEvent")
                    .unchecked_ref::<KeyboardEvent>()
                    .key();
                let mut last: Grid = *self.grid.last().expect("App's grid empty");
                let mut is_direction = true;
                match key.as_ref() {
                    "ArrowUp" => last.move_up(),
                    "ArrowDown" => last.move_down(),
                    "ArrowLeft" => last.move_left(),
                    "ArrowRight" => last.move_right(),
                    _ => is_direction = false,
                }
                if is_direction {
                    last.add_random_2();
                    self.grid.push(last);
                    tx.send(&AppViewOut::Moved(last));
                }
            }
            Restart => (),
            Undo => (),
            _ => (),
        }
    }

    fn view(
        &self,
        tx: &Transmitter<AppModelIn>,
        rx: &Receiver<AppViewOut>,
    ) -> ViewBuilder<HtmlElement> {
        builder!(
            <div class="App" document:keyup=tx.contra_map(|ev: &Event| AppModelIn::KeyUp(Some(ev.clone())))>
                <main>
                    <div patch:children=rx.branch_map(move |AppViewOut::Moved(grid)|
                    Patch::Replace{value: grid::grid_view(grid)
                    , index: 0})>{grid::grid_view(self.grid.last().expect("App grid data empty"))}</div>
                </main>
            </div>
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let gizmo = Gizmo::from(App::default());
    let view = View::from(gizmo.view_builder());
    view.run()
}
