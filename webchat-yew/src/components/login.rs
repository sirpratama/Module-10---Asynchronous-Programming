use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-slate-950 flex w-screen min-h-screen text-white">
            <div class="container mx-auto grid grid-cols-1 lg:grid-cols-2 gap-10 px-8 py-12 items-center">
                <section class="max-w-xl">
                    <p class="text-sm uppercase tracking-wider text-emerald-300">{"Module 10"}</p>
                    <h1 class="text-5xl font-black mt-3 mb-5">{"Rafi's WebChat"}</h1>
                    <p class="text-lg text-slate-300 leading-8">{"A tiny async room for watching websocket messages move between Rust, WASM, and the browser."}</p>
                    <div class="grid grid-cols-3 gap-3 mt-8">
                        <div class="border border-slate-700 p-4 rounded-lg bg-slate-900">
                            <div class="text-2xl font-bold text-emerald-300">{"01"}</div>
                            <div class="text-sm text-slate-400">{"Login"}</div>
                        </div>
                        <div class="border border-slate-700 p-4 rounded-lg bg-slate-900">
                            <div class="text-2xl font-bold text-sky-300">{"02"}</div>
                            <div class="text-sm text-slate-400">{"Chat"}</div>
                        </div>
                        <div class="border border-slate-700 p-4 rounded-lg bg-slate-900">
                            <div class="text-2xl font-bold text-amber-300">{"03"}</div>
                            <div class="text-sm text-slate-400">{"Broadcast"}</div>
                        </div>
                    </div>
                </section>
                <form class="flex w-full max-w-xl">
                    <input {oninput} class="min-w-0 flex-1 rounded-l-lg p-4 border border-slate-700 text-slate-900 bg-white" placeholder="Username" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-emerald-500 hover:bg-emerald-400 disabled:bg-slate-600 disabled:text-slate-300 text-slate-950 font-bold p-4 uppercase border border-emerald-500">
                            {"Enter"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
