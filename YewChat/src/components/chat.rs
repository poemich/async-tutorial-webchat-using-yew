use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User, Route};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleEmojiPicker,
    InsertEmoji(String),
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    show_emoji_picker: bool,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            show_emoji_picker: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message_text = input.value();
                    if !message_text.is_empty() {
                        let message = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(message_text),
                            data_array: None,
                        };
                        if let Err(e) = self
                            .wss
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&message).unwrap())
                        {
                            log::debug!("error sending to channel: {:?}", e);
                        }
                        input.set_value("");
                    }
                };
                false
            }
            Msg::ToggleEmojiPicker => {
                self.show_emoji_picker = !self.show_emoji_picker;
                true
            }
            Msg::InsertEmoji(emoji) => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let current = input.value();
                    input.set_value(&format!("{} {}", current, emoji));
                }
                self.show_emoji_picker = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_emoji = ctx.link().callback(|_| Msg::ToggleEmojiPicker);
        // let logout = ctx.link().callback(|_| Msg::Logout); // Unused
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });
        
        let handle_logout = {
            let history = ctx.link().history().unwrap();
            Callback::from(move |_| {
                history.push(Route::Login);
            })
        };

        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let username = user.username.borrow().clone();
        
        // Find the current user in the list
        let current_user_avatar = self.users.iter()
            .find(|u| u.name == username)
            .map(|u| u.avatar.clone())
            .unwrap_or_else(|| format!(
                "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                username.clone()
            ));
            
        let username_clone = username.clone();

        html! {
            <div class="flex flex-col h-screen w-full bg-gray-100">
                // Header
                <header class="bg-gradient-to-r from-purple-800 to-indigo-800 text-white shadow-lg">
                    <div class="container mx-auto px-4 py-3 flex justify-between items-center">
                        <div class="flex items-center space-x-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd" />
                            </svg>
                            <h1 class="text-2xl font-bold">{"YewChat"}</h1>
                        </div>
                        <div class="flex items-center space-x-4">
                            <div class="flex items-center space-x-2">
                                <img class="w-8 h-8 rounded-full ring-2 ring-white" src={current_user_avatar} alt="Your avatar"/>
                                <span class="font-medium hidden md:inline">{username}</span>
                            </div>
                            <button 
                                onclick={handle_logout}
                                class="bg-white/20 hover:bg-white/30 transition-colors rounded-lg px-3 py-1 text-sm flex items-center"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 001 1h12a1 1 0 001-1V4a1 1 0 00-1-1H3zm1 2v10h10V5H4zm4 7a1 1 0 110-2h2a1 1 0 110 2H8z" clip-rule="evenodd" />
                                </svg>
                                {"Logout"}
                            </button>
                        </div>
                    </div>
                </header>

                // Main Content
                <div class="flex flex-1 overflow-hidden w-full">
                    // Sidebar
                    <div class="hidden md:block w-64 bg-white shadow-md">
                        <div class="p-4 border-b">
                            <h2 class="text-lg font-semibold text-gray-700 flex items-center">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-purple-600" viewBox="0 0 20 20" fill="currentColor">
                                    <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
                                </svg>
                                {"Online Users"}
                                <span class="ml-2 bg-purple-600 text-white text-xs rounded-full px-2 py-1">{self.users.len()}</span>
                            </h2>
                        </div>
                        <div class="overflow-y-auto h-full">
                            {
                                if self.users.is_empty() {
                                    html! {
                                        <div class="p-4 text-center text-gray-500 italic">
                                            {"No users online"}
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <ul class="divide-y">
                                            {
                                                self.users.clone().iter().map(|u| {
                                                    let is_current = u.name == username_clone;
                                                    html!{
                                                        <li class={classes!("p-3", "hover:bg-gray-50", if is_current {"bg-purple-50"} else {""})}>
                                                            <div class="flex items-center space-x-3">
                                                                <div class="relative">
                                                                    <img class="w-10 h-10 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                                                    <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-white"></div>
                                                                </div>
                                                                <div>
                                                                    <div class="text-sm font-medium text-gray-900 flex items-center">
                                                                        {u.name.clone()}
                                                                        {
                                                                            if is_current {
                                                                                html! {
                                                                                    <span class="ml-2 text-xs text-purple-600">{" (you)"}</span>
                                                                                }
                                                                            } else {
                                                                                html! {}
                                                                            }
                                                                        }
                                                                    </div>
                                                                    <div class="text-xs text-gray-500">{"Online"}</div>
                                                                </div>
                                                            </div>
                                                        </li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                    }
                                }
                            }
                        </div>
                    </div>
                    
                    // Chat Area
                    <div class="flex-1 flex flex-col bg-gray-50 w-full">
                        // Messages
                        <div class="flex-1 overflow-y-auto p-4 space-y-4">
                            {
                                if self.messages.is_empty() {
                                    html! {
                                        <div class="flex flex-col items-center justify-center h-full text-gray-500">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-purple-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                            </svg>
                                            <p class="text-xl font-medium">{"No messages yet"}</p>
                                            <p class="text-sm">{"Send a message to start the conversation!"}</p>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <>
                                            {
                                                self.messages.iter().map(|m| {
                                                    let user_option = self.users.iter().find(|u| u.name == m.from);
                                                    
                                                    let default_profile = UserProfile {
                                                        name: m.from.clone(),
                                                        avatar: format!("https://avatars.dicebear.com/api/adventurer-neutral/{}.svg", m.from),
                                                    };
                                                    
                                                    let user = match user_option {
                                                        Some(u) => u,
                                                        None => &default_profile,
                                                    };
                                                    
                                                    let is_current_user = user.name == username_clone;
                                                    let current_time = "now";
                                                    
                                                    html!{
                                                        <div class={classes!(
                                                            "flex", 
                                                            "items-end", 
                                                            "gap-2",
                                                            if is_current_user { "justify-end" } else { "justify-start" }
                                                        )}>
                                                            {
                                                                if !is_current_user {
                                                                    html! {
                                                                        <img class="w-8 h-8 rounded-full" src={user.avatar.clone()} alt="avatar"/>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            <div class={classes!(
                                                                "max-w-md",
                                                                "rounded-2xl",
                                                                "p-3",
                                                                if is_current_user { 
                                                                    classes!("bg-gradient-to-br", "from-purple-600", "to-indigo-600", "text-white", "rounded-br-none")
                                                                } else { 
                                                                    classes!("bg-white", "text-gray-800", "border", "border-gray-200", "rounded-bl-none")
                                                                }
                                                            )}>
                                                                <div class="flex justify-between items-baseline mb-1">
                                                                    <span class={classes!(
                                                                        "font-semibold", "text-sm",
                                                                        if is_current_user { "text-purple-100" } else { "text-purple-600" }
                                                                    )}>
                                                                        {user.name.clone()}
                                                                    </span>
                                                                    <span class={classes!(
                                                                        "text-xs", "ml-2",
                                                                        if is_current_user { "text-purple-200" } else { "text-gray-400" }
                                                                    )}>
                                                                        {current_time}
                                                                    </span>
                                                                </div>
                                                                <div>
                                                                    if m.message.ends_with(".gif") {
                                                                        <img class="rounded-lg max-w-full" src={m.message.clone()}/>
                                                                    } else {
                                                                        <p class="whitespace-pre-wrap break-words">{m.message.clone()}</p>
                                                                    }
                                                                </div>
                                                            </div>
                                                            {
                                                                if is_current_user {
                                                                    html! {
                                                                        <img class="w-8 h-8 rounded-full" src={user.avatar.clone()} alt="avatar"/>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </>
                                    }
                                }
                            }
                        </div>
                        
                        // Input Area
                        <div class="border-t border-gray-200 bg-white p-4 w-full">
                            <div class="flex items-end gap-2 max-w-full">
                                <div class="relative flex-grow">
                                    <input 
                                        ref={self.chat_input.clone()} 
                                        type="text" 
                                        placeholder="Type a message..." 
                                        class="block w-full py-3 px-4 pr-12 bg-gray-100 rounded-full border border-gray-300 focus:border-purple-500 focus:ring-2 focus:ring-purple-200 focus:outline-none transition-all"
                                        onkeypress={onkeypress}
                                    />
                                    <div class="absolute right-2 bottom-2 flex space-x-1">
                                        <button 
                                            onclick={toggle_emoji}
                                            class="p-1 rounded-full hover:bg-gray-200 text-gray-500 hover:text-gray-700 transition-colors"
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </button>
                                    </div>
                                    
                                    {
                                        if self.show_emoji_picker {
                                            let emojis = vec!["😊", "😂", "❤️", "👍", "🔥", "✨", "🎉", "🙏", "💯", "🤔"];
                                            html! {
                                                <div class="absolute right-0 bottom-12 bg-white shadow-lg rounded-lg p-2 border border-gray-200 grid grid-cols-5 gap-1">
                                                    {
                                                        emojis.iter().map(|emoji| {
                                                            let emoji_clone = emoji.to_string();
                                                            let insert_emoji = ctx.link().callback(move |_| Msg::InsertEmoji(emoji_clone.clone()));
                                                            html! {
                                                                <button 
                                                                    onclick={insert_emoji} 
                                                                    class="w-8 h-8 text-xl hover:bg-gray-100 rounded"
                                                                >
                                                                    {emoji}
                                                                </button>
                                                            }
                                                        }).collect::<Html>()
                                                    }
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                                <button 
                                    onclick={submit}
                                    class="bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white rounded-full p-3 shadow-md hover:shadow-lg transition-all flex-shrink-0"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" transform="rotate(90 12 12)" />
                                    </svg>
                                </button>
                            </div>
                            <div class="text-xs text-gray-500 mt-2 text-center">
                                {"Pro tip: Type .gif at the end of your message to show an image!"}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}