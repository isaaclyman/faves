use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div class="p-8 flex-1 bg-rust text-black">
        <h1 class="text-xl font-light pb-4">{ "Welcome." }</h1>
        <p>{ "This site was built with type-safe, memory-safe Rust!" }</p>
        <p>{ "There's no JavaScript in the input bundle. All functionality is enabled by WebAssembly." }</p>
        <p class="text-xs">{ "(There's about 25kb of JavaScript in the output bundle for interop.)" }</p>
    </div>
  }
}