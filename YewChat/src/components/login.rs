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
       <div class="bg-gradient-to-br from-purple-800 to-indigo-900 flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-8 shadow-2xl max-w-md w-full">
                    <div class="flex justify-center mb-6">
                        <div class="text-white text-4xl font-bold flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 mr-2 text-purple-300" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd" />
                            </svg>
                            {"YewChat"}
                        </div>
                    </div>
                    <div class="text-white text-center mb-6">
                        {"Connect with friends and join the conversation!"}
                    </div>
                    
                    <div class="animate-bounce mx-auto w-20 h-20 mb-8 flex justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-full w-full text-purple-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z" />
                        </svg>
                    </div>
                    
                    <form class="flex flex-col">
                        <div class="relative mb-4">
                            <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-purple-300" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <input 
                                {oninput} 
                                class="bg-white/20 text-white placeholder-purple-200 rounded-lg p-4 pl-10 w-full border border-purple-400/30 focus:border-purple-400 focus:outline-none focus:ring-2 focus:ring-purple-400/50 transition-all" 
                                placeholder="Enter your username" 
                            />
                        </div>
                        <Link<Route> to={Route::Chat} classes="w-full"> 
                            <button 
                                {onclick} 
                                disabled={username.len()<1} 
                                class="w-full px-8 rounded-lg bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white font-bold p-4 uppercase shadow-lg hover:shadow-xl transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                {"Start Chatting!"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}