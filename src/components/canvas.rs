use super::signals::Signals;
use super::settings::Settings;
use yew::{html, Component, Context, Html, Properties};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Msg {
    Tick,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
}

#[derive(Debug)]
pub struct CANVAS {
    signals: Vec<Signals>,
    node_ref: NodeRef,
}
impl Component for CANVAS {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let settings = &ctx.props().settings;
        let signals = (0..40)
            .map(|idx| Signals::new_random(settings, 100+idx*20, idx%3))
            .collect();

        Self { signals, node_ref: NodeRef::default(), }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {

        let settings = &ctx.props().settings;

        self.signals.iter_mut().for_each(|s| s.update(&settings));

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <canvas ref={self.node_ref.clone()} height="1500" width="2500"></canvas>
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

        for signal in &self.signals {
            context.move_to(signal.show[0].1 as f64,signal.show[0].0 as f64);
            for (a,b) in &signal.show {
                context.line_to(*b as f64,*a as f64);
            }
            if signal.color == 0 {
                context.set_fill_style(&JsValue::from_str("red"));
            } else if signal.color == 1 {
                context.set_fill_style(&JsValue::from_str("green"));
            } else {
                context.set_fill_style(&JsValue::from_str("blue"));
            }
        }

        context.stroke();
    }
}
