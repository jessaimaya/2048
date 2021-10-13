use wasm_bindgen::{closure, JsCast, JsValue};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use mogwai::prelude::*;

use crate::components::metaball::*;

#[derive(Clone)]
pub enum In {
    Click,
    CanvasIn(web_sys::HtmlElement),
}

#[derive(Clone)]
pub enum Out {
    DrawClicks(i32),
}

pub struct Home {
    pub num_clicks: i32,
    pub ctx: Rc<RefCell<Option<web_sys::CanvasRenderingContext2d>>>,
}

impl Component for Home {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        match msg {
            In::Click => {
                self.num_clicks += 1;
                tx_view.send(&Out::DrawClicks(self.num_clicks));
            }
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
                self.ctx = Rc::new(RefCell::new(Some(context.clone())));

                let c_w = body().client_width();
                let c_h = body().client_height();

                // let mut lava0 = LavaLamp::new(
                //     c_w as f32,
                //     c_h as f32,
                //     10,
                //     String::from("#5d3a97"),
                //     String::from("#8942a4"),
                //     self.ctx.to_owned(),
                // );

                let mut lava1 = LavaLamp::new(
                    c_w as f32,
                    c_h as f32,
                    10,
                    String::from("#24519f"),
                    String::from("#fa0000"),
                    self.ctx.borrow_mut().take(),
                );
                let mut lava2 = LavaLamp::new(
                    c_w as f32,
                    c_h as f32,
                    10,
                    String::from("#60bfbd"),
                    String::from("#1c4995"),
                    self.ctx.borrow_mut().take()
                    );

                request_animation_frame(move |_t| {
                    context.clear_rect(0.0, 0.0, c_w as f64, c_h as f64);
                    //lava0.render_metaball();
                     lava1.render_metaball();
                     lava2.render_metaball();
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
                <div class="home__top">
                    <canvas
                        width={format!("{}",w)}
                        height={format!("{}",h.as_f64().unwrap())}
                        id="canvas"
                        class="lavalamp_canvas"
                        post:build = tx.contra_map(|canvas: &HtmlElement| In::CanvasIn(canvas.clone()))
                    />
                </div>
                <div 
                    on:click=tx.contra_map(|_| In::Click)
                    class="bubble main green"
                >
                        <h1 class="buble__title">"2048"</h1>
                        <h2 class="bubble__subtitle">"mogwai"</h2>
                        <img src="./play.png" width = "" />
                    <button >
                        {(
                            "clicks = 1",
                            rx.branch_map(|msg| {
                                match msg {
                                    Out::DrawClicks(n) => {
                                        format!("clicks = {}", n)
                                    }
                                }
                            })
                        )}
                    </button>
                    </div>
            </div>
        }
    }
}
