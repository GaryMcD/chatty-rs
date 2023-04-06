mod chatbox;
mod input;
mod messages;

use crate::chatbox::ChatBox;
use yew::prelude::*;

const USER_ONE: &str = "Steve";
const USER_TWO: &str = "Frank";
const PAGE_CONTAINER_CLASS: &str = "flex flex-row gap-10 justify-center w-screen min-h-screen bg-gray-100 text-gray-800 p-10";

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <PageContainer/>
    }
}

// https://codepen.io/robstinson/pen/oNLaLMN
#[cfg(debug_assertions)]
#[function_component]
fn PageContainer() -> Html {
    html! {
        <div class = {PAGE_CONTAINER_CLASS.to_owned()}>
            <ChatBox 
                user = {USER_ONE.to_owned()} />
            <ChatBox 
                user = {USER_TWO.to_owned()} />
        </div>
    }
}

#[cfg(not(debug_assertions))]
#[function_component]
fn PageContainer() -> Html {
    let messages_handle = use_state(Vec::new);

    let on_message = {
        let messages_handle = messages_handle.clone();
        Callback::from(move |message| {
            let mut old = (*messages_handle).clone();
            old.push(message);
            messages_handle.set(old);
        })
    };
    
    html! {
        <div class = {PAGE_CONTAINER_CLASS.to_owned()}>
            <ChatBox 
                user = {USER_ONE.to_owned()}
                chat_history = {(*messages_handle).clone()}
                on_message = {on_message.clone()} />
            <ChatBox 
                user = {USER_TWO.to_owned()}
                chat_history = {(*messages_handle).clone()}
                on_message = {on_message} />
        </div>
    }
}