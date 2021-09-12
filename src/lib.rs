use game::*;
use log::info;
use log::Level;
use mogwai::prelude::*;
use std::default;
use std::panic;
use wasm_bindgen::prelude::*;

mod components;
mod game;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Grid = [[u8; 4]; 4];

#[derive(Debug)]
struct App {
    grid: Grid,
}

#[derive(Clone)]
enum AppModel {}

#[derive(Clone)]
enum AppView {}

impl Default for App {
    fn default() -> Self {
        let mut grid: Grid = [[0; 4]; 4];
        game::add_random_2(&mut grid);
        App { grid }
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, _msg: &AppModel, _tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        /*match msg {

        }*/
    }

    fn view(
        &self,
        _tx: &Transmitter<AppModel>,
        _rx: &Receiver<AppView>,
    ) -> ViewBuilder<HtmlElement> {
        builder!(
            <div class="App">
                <main>
                    <p>"Main content"</p>
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
    info!("Initial app: {:?}", gizmo.state);
    let view = View::from(gizmo.view_builder());
    view.run()
}
