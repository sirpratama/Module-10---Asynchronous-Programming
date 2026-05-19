use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
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

        if wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
            .is_ok()
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
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
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
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
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen bg-slate-50">
                <div class="flex-none w-72 h-screen bg-slate-950 text-white">
                    <div class="p-5 border-b border-slate-800">
                        <div class="text-xl font-bold">{"Rafi's WebChat"}</div>
                        <div class="text-xs text-slate-400 mt-1">{"Async lounge"}</div>
                    </div>
                    <div class="text-sm uppercase tracking-wider text-slate-400 p-5 pb-2">{"Online crew"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex mx-4 my-3 bg-slate-900 border border-slate-800 rounded-lg p-2">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-emerald-300">
                                            {"Ready"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    <div class="mx-4 mt-6 p-4 border border-slate-800 rounded-lg bg-slate-900">
                        <div class="text-sm font-semibold text-amber-300">{"Creative note"}</div>
                        <p class="text-xs text-slate-400 mt-2 leading-5">{"Messages that end with .gif render as images, while plain text stays compact."}</p>
                    </div>
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-16 border-b border-slate-200 bg-white flex items-center justify-between px-6">
                        <div>
                            <div class="text-xl font-bold text-slate-900">{"Chat"}</div>
                            <div class="text-xs text-slate-500">{"127.0.0.1:8080"}</div>
                        </div>
                        <div class="text-xs bg-emerald-100 text-emerald-700 px-3 py-1 rounded-full">{"Connected"}</div>
                    </div>
                    <div class="w-full grow overflow-auto border-b border-slate-200 bg-white">
                        if self.messages.is_empty() {
                            <div class="h-full flex items-center justify-center text-slate-400">
                                <div class="text-center">
                                    <div class="text-2xl font-bold text-slate-600">{"No messages yet"}</div>
                                    <div class="text-sm mt-2">{"The room is quiet."}</div>
                                </div>
                            </div>
                        }
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-end w-3/6 bg-slate-100 m-8 rounded-tl-lg rounded-tr-lg rounded-br-lg border border-slate-200">
                                        <img class="w-8 h-8 rounded-full m-3" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm font-semibold text-slate-800">
                                                {m.from.clone()}
                                            </div>
                                            <div class="text-xs text-slate-500">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full h-16 flex px-3 items-center bg-slate-50">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-3 pl-4 mx-3 bg-white border border-slate-200 rounded-full outline-none focus:text-slate-700 focus:border-emerald-400" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm bg-emerald-500 hover:bg-emerald-400 w-11 h-11 rounded-full flex justify-center items-center color-white">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
