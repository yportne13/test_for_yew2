mod components;
pub mod macros;

use crate::components::{
    Components, Home,
};
use material_yew::{
    drawer::{MatDrawerAppContent, MatDrawerTitle},
    top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle},
    MatButton, MatDrawer, MatIconButton, MatTopAppBarFixed,
};
use yew::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/components")]
    Components,
    #[at("/")]
    Home,
}

type AppLink = Link<AppRoute>;

pub struct App {
    /// `true` represents open; `false` represents close
    drawer_state: bool,
}

pub enum Msg {
    NavIconClick,
    Opened,
    Closed,
}

pub struct SyntectData {
    pub theme: Option<Theme>,
    pub syntax_set: Option<SyntaxSet>,
}

thread_local!(pub static SYNTECT_DATA: RefCell<SyntectData> = RefCell::new(SyntectData {
    theme: None,
    syntax_set: None,
}));

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            drawer_state: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NavIconClick => {
                self.drawer_state = !self.drawer_state;
                true
            }
            Msg::Closed => {
                self.drawer_state = false;
                false
            }
            Msg::Opened => {
                self.drawer_state = true;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let is_on_mobile = is_on_mobile();

        let components = if !is_on_mobile {
            html! { <MatButton label="Components"/>}
        } else {
            html! {
                 <MatIconButton label="Components">
                     <img src="/assets/components.png" alt="Components" />
                 </MatIconButton>
            }
        };

        let docs = if !is_on_mobile {
            html! { <MatButton label="API Docs"/>}
        } else {
            html! { <MatIconButton icon="description" label="API Docs" />}
        };

        let github = if !is_on_mobile {
            html! { <MatButton label="GitHub" />}
        } else {
            html! {
                 <MatIconButton label="GitHub">
                     <img src="/assets/github.png" alt="GitHub logo" />
                 </MatIconButton>
            }
        };

        html! { <>
        <BrowserRouter>
            <MatDrawer open={self.drawer_state} drawer_type="dismissible"
                onopened={link.callback(|_| Msg::Opened)}
                onclosed={link.callback(|_| Msg::Closed)}>

                    <MatDrawerTitle>
                        <span class="drawer-title">{"Components"}</span>
                    </MatDrawerTitle>

                    <div class="drawer-content">
                    </div>
                    <MatDrawerAppContent>
                        <div class="app-content">
                            <MatTopAppBarFixed onnavigationiconclick={link.callback(|_| Msg::NavIconClick)}>
                                <MatTopAppBarNavigationIcon>
                                    <MatIconButton icon="menu"></MatIconButton>
                                </MatTopAppBarNavigationIcon>

                                <MatTopAppBarTitle>
                                    <div class="app-title">
                                        <AppLink to={AppRoute::Home}>
                                            <h1>{"Material Yew"}</h1>
                                        </AppLink>
                                        <span class="action-item">
                                            <AppLink to={AppRoute::Components}>
                                                {components}
                                            </AppLink>
                                        </span>
                                    </div>
                                </MatTopAppBarTitle>

                                <MatTopAppBarActionItems>
                                    <a class="action-item" href="https://github.com/hamza1311/yew-material">
                                        {github}
                                    </a>
                                </MatTopAppBarActionItems>

                                <MatTopAppBarActionItems>
                                    <a class="action-item" href="/docs/material_yew">
                                        {docs}
                                    </a>
                                </MatTopAppBarActionItems>

                            </MatTopAppBarFixed>
                            <main id="router-outlet">
                                <yew_router::Switch<AppRoute> render={yew_router::Switch::render(Self::switch)} />
                            </main>
                        </div>
                    </MatDrawerAppContent>
                </MatDrawer>
            </BrowserRouter>
        </>}
    }
}

impl App {
    fn switch(switch: &AppRoute) -> Html {
        match switch {
            AppRoute::Home => html! { <Home />},
            AppRoute::Components => html! { <Components />},
        }
    }
}

pub fn is_on_mobile() -> bool {
    gloo_utils::window()
        .match_media("(max-width: 600px)")
        .unwrap()
        .unwrap()
        .matches()
}
