use wasm_bindgen::JsCast;

use mogwai::prelude::*;

use crate::components::metaball::*;

#[derive(Clone)]
pub enum In {
    CanvasIn(web_sys::HtmlElement),
}

#[derive(Clone)]
pub enum Out {}

pub struct Home {
    pub ctx: Option<web_sys::CanvasRenderingContext2d>,
}

impl Component for Home {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        match msg {
            In::CanvasIn(canvas) => {
                let canvas = canvas
                    .to_owned()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();
                self.ctx = Some(context.clone());

                let c_w = body().client_width();
                let c_h = body().client_height();

                let mut lava1 = LavaLamp::new(
                    c_w as f32,
                    c_h as f32,
                    10,
                    String::from("#24519f"),
                    String::from("#fa0000"),
                    self.ctx.to_owned(),
                );
                let mut lava2 = LavaLamp::new(
                    c_w as f32,
                    c_h as f32,
                    10,
                    String::from("#60bfbd"),
                    String::from("#1c4995"),
                    self.ctx.to_owned(),
                );

                request_animation_frame(move |_t| {
                    context.clear_rect(0.0, 0.0, c_w as f64, c_h as f64);
                    lava2.render_metaball();
                    lava1.render_metaball();
                    true
                });
            }
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let w = body().client_width() as usize;
        let h = window().inner_height().unwrap();
        builder! {
            <div class="home">
                <canvas
                    width=format!("{}",w)
                    height=format!("{}",h.as_f64().unwrap())
                    id="canvas"
                    class="lavalamp_canvas"
                    post:build = tx.contra_map(|canvas: &HtmlElement| In::CanvasIn(canvas.clone()))
                />
                <div class="black-screen"></div>
                <div class="bubble main green">
                    <h1 class="buble__title">"2048"</h1>
                    <h2 class="bubble__subtitle">"mogwai"</h2>
                </div>
                <a class="button green start" href="#/play">"Start game"</a>
            </div>
        }
    }
}
