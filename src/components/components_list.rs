use yew::prelude::*;
use web_sys::{console, HtmlElement, MouseEvent};
use std::f64;
use wasm_bindgen::JsCast;

pub struct Components {
    node_ref: NodeRef,
}

impl Component for Components {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {

        //let onmousemove = Callback::from(|e: MouseEvent| {
        //    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        //        let rect = target.get_bounding_client_rect();
        //        let x = (e.client_x() as f64) - rect.left();
        //        let y = (e.client_y() as f64) - rect.top();
        //        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        //    }
        //});

        

        html! {
          <section class="components-grid">
            //<div id="mousemoveme" {onmousemove}></div>
            <canvas ref={self.node_ref.clone()} height="500" width="2500"></canvas>
          </section>
        }  
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.node_ref.cast::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        context.stroke();
    }
}
