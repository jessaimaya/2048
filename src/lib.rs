use log::info;
use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

use web_sys::HashChangeEvent;

mod components;
mod containers;
mod router;

use crate::router::Route;
use components::grid::*;
use containers::home::Home;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    route: Route,
}

impl Default for App {
    fn default() -> Self {
        App { route: Route::Home }
    }
}

#[derive(Clone)]
enum AppModel {
    HashChange(String),
}

#[derive(Clone)]
enum AppView {
    PatchPage(Patch<View<HtmlElement>>),
    Error(String),
}

impl AppView {
    fn error(&self) -> Option<String> {
        match self {
            AppView::Error(msg) => Some(msg.clone()),
            _ => None,
        }
    }

    /// If the message is a new route, convert it into a patch to replace the current main page.
    fn patch_page(&self) -> Option<Patch<View<HtmlElement>>> {
        match self {
            AppView::PatchPage(patch) => Some(patch.clone()),
            _ => None,
        }
    }
}

impl Component for App {
    type ModelMsg = AppModel;
    type ViewMsg = AppView;
    type DomNode = HtmlElement;

    fn bind(&self, input_sub: &Subscriber<AppModel>) {
        info!("BINDING");
        let perf_nav = window().performance().expect("Performance not supported")
            .navigation();
        let hash = window().location().hash().expect("Error getting location hash");

        info!("PerfNav: {}, hash:{}", perf_nav.type_(), hash);
 //        if perf_nav.type_() == web_sys::PerformanceNavigation::TYPE_RELOAD {
            input_sub.send_async(async move {
                AppModel::HashChange(hash)
            });
            //
   //     }
   
         if perf_nav.type_() == web_sys::PerformanceNavigation::TYPE_BACK_FORWARD {
             info!("BACKWARD!");
         }
    }

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::HashChange(hash) => {
                // When we get a hash change, attempt to convert it into one of our routes

                if hash.is_empty() {
                    // Force empty hash
                    window().location().set_hash("/").expect("Couldn't redirect to #/");
                }

                match Route::try_from(hash.as_str()) {
                    // If we can't, let's send an error message to the view
                    Err(msg) => tx.send(&AppView::Error(msg)),
                    // If we _can_, create a new view from the route and send a patch message to
                    // the view
                    Ok(route) => {
                        if route != self.route {
                            let view = View::from(ViewBuilder::from(&route));
                            self.route = route;
                            tx.send(&AppView::PatchPage(Patch::Replace {
                                index: 2,
                                value: view,
                            }));
                        }
                    }
                }
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <slot
                class="App"
                window:hashchange=tx.contra_filter_map(|ev:&Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)>
                <nav>
                    <ul>
                        <li class=self.route.nav_home_class()>
                            <a href=String::from(Route::Home)>"Home"</a>
                        </li>
                        <li class=self.route.nav_play_class()>
                            <a href=String::from(Route::Play)>"Play"</a>
                        </li>
                    </ul>
                </nav>
                <pre class="pre-error">{rx.branch_filter_map(AppView::error)}</pre>
                {ViewBuilder::from(&self.route)}
            </slot>
        }
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
