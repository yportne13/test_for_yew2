use super::signals::Signals;
use super::settings::Settings;
use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

#[derive(Debug)]
pub enum Msg {
    Tick,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
}

#[derive(Debug)]
pub struct SVG {
    signals: Vec<Signals>,
}
impl Component for SVG {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let settings = &ctx.props().settings;
        let signals = (0..40)
            .map(|idx| Signals::new_random(settings, 100+idx*20, idx%3))
            .collect();

        Self { signals }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {

        let settings = &ctx.props().settings;

        self.signals.iter_mut().for_each(|s| s.update(&settings));

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", 1600, 1000);

        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.signals.iter().map(Signals::render) }
            </svg>
        }
    }
}
