use yew::prelude::*;

pub struct Components {}

impl Component for Components {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
          <section class="components-grid">
          </section>
        }
    }
}
