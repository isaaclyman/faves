use yew::{function_component, Html, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {
    <div class="p-4 flex-1 bg-rust text-black">
        <h1>{ "Page not found." }</h1>
    </div>
  }
}