use web_sys::CustomEvent;
use yew::prelude::*;
//use web_sys::{console, HtmlElement, MouseEvent};
//use std::f64;
//use wasm_bindgen::JsCast;
use super::settings::Settings;
use super::svg::SVG;
use super::canvas::CANVAS;
use super::slider::Slider;
use material_yew::MatSwitch;

pub enum Msg {
    SVGorCanvas(bool),
    ChangeSize(i32),
    ChangeX(i32),
}

pub struct Components {
    settings: Settings,
    isSVG: bool,
}

impl Component for Components {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            settings: Settings{size: 1, x_axis: 0, },
            isSVG: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SVGorCanvas(x) => {
                self.isSVG = x;
                true
            }
            Msg::ChangeSize(x) => {
                self.settings.size = x;
                true
            }
            Msg::ChangeX(x) => {
                self.settings.x_axis = x;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        //let onmousemove = Callback::from(|e: MouseEvent| {
        //    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        //        let rect = target.get_bounding_client_rect();
        //        let x = (e.client_x() as f64) - rect.left();
        //        let y = (e.client_y() as f64) - rect.top();
        //        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        //    }
        //});

        let svg = html! {
            <SVG settings={self.settings.clone()} />
        };
        let canvas = html! {
            <CANVAS settings={self.settings.clone()} />
        };

        html! {
          <section>
            <Slider label="size"
                    min=1.0 max=20.0
                    onchange={link.callback(|x| Msg::ChangeSize(x as i32))}
                    value={self.settings.size as f64} />
            <Slider label="x_axis"
                    min=0.0 max=3000.0
                    onchange={link.callback(|x| Msg::ChangeX(x as i32))}
                    value={self.settings.x_axis as f64} />
            <MatSwitch checked={self.isSVG} onchange={link.callback(|x| Msg::SVGorCanvas(x))} />
            //<div id="mousemoveme" {onmousemove}></div>
            //
            {if self.isSVG {
                svg
            }else {
                canvas
            }}
          </section>
        }  
    }

}
