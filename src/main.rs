use serde_json::Value;
use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

mod category;
mod data;
mod home;
mod icons;
mod not_found;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:name")]
    Category { name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
    CloseNavbar,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Category { name } => html! {
            <category::Category name={name.to_owned()}  />
        },
        Route::NotFound => html! { <not_found::NotFound /> },
    }
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
            Msg::CloseNavbar => {
                self.navbar_active = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <HashRouter>
                { self.view_header(ctx.link()) }
                { self.view_body(||
                    html! {
                        <Switch<Route> render={switch} />
                    }
                , ctx.link()) }
            </HashRouter>
        }
    }
}

impl HtmlModel {
    fn view_header(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = self;

        html! {
            <header
                class="bg-blue text-white py-4 px-4 flex flex-row items-baseline border-b-2 border-black z-20"
            >
                <button class={classes!(
                    "transition-colors",
                    if *navbar_active {"bg-slate-700"} else {"bg-slate-600"},
                    if *navbar_active {"text-slate-300"} else {"text-slate-100"},
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
                <h1 class="text-xl pl-6 font-light">{ "a few of my favorite things" }</h1>
            </header>
        }
    }

    fn view_body<F: FnOnce() -> Html>(&self, outlet: F, link: &Scope<Self>) -> Html {
        let Self {
            navbar_active,
            category_data,
            ..
        } = self;

        html! {
            <main class="flex-1 relative flex flex-col z-10">
                <div class={classes!(
                    "absolute",
                    "z-1",
                    "inset-0",
                    "bg-white/75",
                    "transition-opacity",
                    if *navbar_active {"opacity-100"} else {"opacity-0"},
                    if *navbar_active {"pointer-events-auto"} else {"pointer-events-none"}
                )} onclick={link.callback(|_| Msg::CloseNavbar)}>
                </div>
                <nav class={classes!(
                    "flex",
                    "flex-col",
                    "justify-start",
                    "items-start",
                    "bg-blue",
                    "text-white",
                    "py-4",
                    "nav-menu",
                    "z-5",
                    if *navbar_active {"nav-menu-active"} else {""}
                )}
                    role="navigation" aria-label="main navigation">
                    <div class="flex-1 px-4 flex flex-col" onclick={link.callback(|_| Msg::CloseNavbar)}>
                        <Link<Route> classes={classes!("navbar-item", "text-lg")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <hr class="my-2 border-white" />
                        { category_data.iter().map(|(name, _)|
                            html! {
                                <Link<Route>
                                    classes={classes!("navbar-item", "text-lg")}
                                    to={Route::Category { name: name.to_owned() }}>
                                    { name }
                                </Link<Route>>
                            }
                        ).collect::<Html>() }
                    </div>
                    <footer class="text-xs font-bold mx-2 px-2 border-l-4 border-lightblue nav-footer">
                        { "Built with " }
                        <a href="https://webassembly.org/">{ "WebAssembly" }</a>
                        { " (via " }
                        <a href="https://www.rust-lang.org/">{ "Rust" }</a>
                        { " & " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { ") and "}
                        <a href="https://tailwindcss.com/">{ "Tailwind" }</a>
                    </footer>
                </nav>
                { outlet() }
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<HtmlModel>::new().render();
}
