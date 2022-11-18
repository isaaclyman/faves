use serde_json::Value;
use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

mod category;
mod data;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/faves/:category")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <category::Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

pub enum Msg {
    ToggleNavbar,
}

pub struct HtmlModel {
    navbar_active: bool,
    category_data: Vec<(String, Value)>,
}

impl Component for HtmlModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let data_map = data::get_all_data();
        let mut category_data = data_map
            .iter()
            .map(|(key, val)| (key.to_owned(), val.to_owned()))
            .collect::<Vec<(String, Value)>>();
        category_data.sort_by_key(|(key, _)| key.to_owned());
        Self {
            navbar_active: false,
            category_data,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_header(ctx.link()) }
                { self.view_body(||
                    html! {
                        <Switch<Route> render={Switch::render(switch)} />
                    }
                ) }
            </BrowserRouter>
        }
    }
}

impl HtmlModel {
    fn view_header(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = self;

        html! {
            <header
                class="bg-slate-900 text-slate-400 py-6 px-4 flex flex-row items-baseline border-b-2 border-blue-800"
            >
                <button class={classes!(
                    "transition-colors",
                    if *navbar_active {"bg-slate-600"} else {"bg-slate-700"},
                    if *navbar_active {"text-slate-100"} else {"text-slate-300"},
                    "rounded-full",
                    "py-1",
                    "px-4",
                    "text-sm",
                )}
                    aria-label="menu" aria-expanded={if *navbar_active {"true"} else {"false"}}
                    onclick={link.callback(|_| Msg::ToggleNavbar)}
                >
                    { "menu" }
                </button>
                <h1 class="text-2xl pl-6 font-light">{ "a few of my favorite things" }</h1>
            </header>
        }
    }

    fn view_body(&self, outlet: fn() -> Html) -> Html {
        let Self {
            navbar_active,
            category_data,
            ..
        } = self;

        html! {
            <main class="flex-1 relative flex flex-col">
                <nav class={classes!(
                    "flex",
                    "flex-col",
                    "justify-start",
                    "items-start",
                    "bg-slate-900",
                    "text-slate-400",
                    "py-4",
                    "nav-menu",
                    if *navbar_active {"nav-menu-active"} else {""}
                )}
                    role="navigation" aria-label="main navigation">
                    <div class="flex-1 px-4 flex flex-col">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <hr class="my-2 border-slate-500" />
                        { category_data.iter().map(|(name, _)|
                            html! {
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Secure}>
                                    { name }
                                </Link<Route>>
                            }
                        ).collect::<Html>() }
                    </div>
                    <footer class="text-xs font-bold mx-2 px-2 border-l-4 border-blue-800 nav-footer">
                        { "Built with " }
                        <a href="https://www.rust-lang.org/">{ "Rust" }</a>
                        { ", " }
                        <a href="https://webassembly.org/">{ "WebAssembly" }</a>
                        { ", " }
                        <a href="https://tailwindcss.com/">{ "Tailwind" }</a>
                        { " and "}
                        <a href="https://yew.rs">{ "Yew" }</a>
                    </footer>
                </nav>
                { outlet() }
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<HtmlModel>();
}
