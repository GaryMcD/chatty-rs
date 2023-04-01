use chrono::{DateTime, Utc};
use crate::{input::MessageInput, messages::{Messages, MessageType, MessageProperties}};
use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use shared::Message;
use std::{time::Duration, vec::Vec};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const CHATBOX_CONTAINER_CLASS: &str = "flex flex-col flex-grow w-full max-w-xl bg-white shadow-xl rounded-lg overflow-hidden";

#[derive(Properties, PartialEq)]
pub(crate) struct ChatBoxProperties {
    pub user: String,
}

#[function_component]
pub(crate) fn ChatBox(props: &ChatBoxProperties) -> Html {
    let chat_history_handle = use_state(Vec::new);
    let chat_history_value = (*chat_history_handle).clone();
    
    let user = props.user.clone();
    use_effect_with_deps(move |_| {
        let user = user.clone();
        let interval = generate_chat_history_retrieval_interval(chat_history_handle, user);

        move || { interval.cancel(); ()}
    }, ());

    let on_message = on_message_callback(props.user.clone());

    html! {
        <div class = {CHATBOX_CONTAINER_CLASS.to_owned()}>
            <Messages
                messages = {chat_history_value} />
            <MessageInput 
                on_message = {on_message} />
        </div>
    }
}

fn convert_to_message_properties(messages: &[Message], user: &str) -> Vec<MessageProperties> {
    messages.iter()
        .map(|message| {
            MessageProperties { 
                message: message.message.clone().into(), 
                message_type: get_message_type(user, &message.user), 
                time: message.time_passed().into() 
            }
        })
        .collect()
}

fn get_message_type(chatbox_user: &str, message_user: &str) -> MessageType {
    if message_user == chatbox_user { 
        MessageType::Outgoing 
    } else { 
        MessageType::Incoming 
    }
}

fn generate_chat_history_retrieval_interval(chat_history_handle: UseStateHandle<Vec<MessageProperties>>, user: String) -> Interval {
    Interval::new(Duration::from_millis(100).as_millis() as u32, move || {
        let chat_history_handle = chat_history_handle.clone();
        let user = user.clone();
        spawn_local(async move {
            let chat_history_raw: String = 
                Request::get("http://127.0.0.1:9090/")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let as_array: Vec<Message> = serde_json::from_str(&chat_history_raw).unwrap();
            let converted = convert_to_message_properties(&as_array, &user);
            chat_history_handle.set(converted);
        });
    })
}

fn on_message_callback(user: String) -> Callback<String> {
    Callback::from(move |message_text: String| {
        let user = user.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let message = Message {
                message: message_text,
                user: user,
                time: DateTime::<Utc>::from(Utc::now()),
            };
            let _ = Request::post("http://127.0.0.1:9090/")
                .body(serde_json::to_string(&message).unwrap())
                .send()
                .await;
        });
    })
}