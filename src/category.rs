use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(super::Route::Home));
    html! {
        <div class="p-4 flex-1 bg-slate-600 text-slate-100">
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}