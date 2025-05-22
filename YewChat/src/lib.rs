#![recursion_limit = "512"]

mod components;
mod services;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::chat::Chat;
use components::login::Login;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex w-screen h-screen bg-gray-50">
                    <Switch<Route> render={Switch::render(switch)}/>
                </div>
            </BrowserRouter>
        </ContextProvider<User>>
    }
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {
            <div class="flex flex-col items-center justify-center h-screen bg-gradient-to-br from-purple-800 to-indigo-900 text-white p-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-24 w-24 text-purple-300 mb-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <h1 class="text-4xl font-bold mb-2">{"404"}</h1>
                <p class="text-xl mb-6">{"Page not found"}</p>
                <Link<Route> to={Route::Login} classes="bg-white/20 hover:bg-white/30 text-white px-6 py-3 rounded-lg transition-colors">
                    {"Back to Login"}
                </Link<Route>>
            </div>
        },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}