use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="p-8 flex-1 bg-rust text-black">
          <h1 class="text-xl font-light pb-4">{ "Welcome." }</h1>
          <p>{ "You've discovered an archive of all the things I love. Feel free to browse around." }</p>
          <hr class="my-4 border-jet" />
          <h1 class="text-xl font-light pb-4">{ "Technical Details" }</h1>
          <p class="mb-2">{ "This site was built with type-safe, memory-safe Rust!" }</p>
          <p class="mb-1">{ "There's no JavaScript in the input bundle. All functionality is enabled by WebAssembly." }</p>
          <p class="text-xs mb-2">{ "(There's about 25kb of JavaScript in the output bundle for interop.)" }</p>
          <p class="mb-2">{ "Every page (except this one) is data-driven by JSON embedded in the site binary." }</p>
      </div>
    }
}
