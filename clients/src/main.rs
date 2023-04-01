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

#[function_component]
// https://codepen.io/robstinson/pen/oNLaLMN
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