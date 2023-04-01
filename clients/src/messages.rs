use std::vec::Vec;
use yew::prelude::*;

const MESSAGE_CONTAINER_BASE_CLASS: &str = "flex w-full mt-2 space-x-3 max-w-xs"; 
const MESSAGE_CONTAINER_OUTGOING_ADDITIONAL_CLASS: &str = "ml-auto justify-end";
const MESSAGE_CONTENT_INCOMING_CLASS: &str = "bg-gray-300 p-3 rounded-r-lg rounded-bl-lg";
const MESSAGE_CONTENT_OUTGOING_CLASS: &str = "bg-blue-600 text-white p-3 rounded-l-lg rounded-br-lg";
const MESSAGE_TEXT_CLASS: &str = "text-sm";
const MESSAGES_CONTAINER_CLASS: &str = "flex flex-col flex-grow h-0 p-4 overflow-auto";
const TIME_TEXT_CLASS: &str = "text-xs text-gray-500 leading-none";
const USER_ICON_CLASS: &str = "flex-shrink-0 h-10 w-10 rounded-full bg-gray-300";

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct MessagesProperties {
    pub messages: Vec<MessageProperties>,
}

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct MessageProperties {
    pub message: AttrValue,
    pub message_type: MessageType,
    pub time: AttrValue,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum MessageType {
    Incoming,
    Outgoing
}

#[function_component]
pub(crate) fn Messages(props: &MessagesProperties) -> Html {
    html! {
        <div class = {MESSAGES_CONTAINER_CLASS.to_owned()}>
            {
                props.messages.iter()
                    .map(|message| {
                        html! { 
                            <Message
                                message_type = {message.message_type}
                                message = {message.message.clone()}
                                time = {message.time.clone()} /> 
                        }})
                    .collect::<Html>()
            }
        </div>
    }
}

#[function_component]
fn Message(props: &MessageProperties) -> Html {
    let outer_div_class = get_message_class_outer_div(props.message_type);
    let message_content_div_class = get_message_class_content_div(props.message_type);
    let display_icon_first = props.message_type == MessageType::Incoming;

    html! {
        <div class = {outer_div_class}>
            if display_icon_first { <div class = {USER_ICON_CLASS.to_owned()} /> }
            <div>
                <div class = {message_content_div_class}>
                    <p class = {MESSAGE_TEXT_CLASS.to_owned()}> {&props.message} </p>
                </div>
                <span class = {TIME_TEXT_CLASS.to_owned()}>
                    {&props.time}
                </span>
            </div>
            if !display_icon_first { <div class = {USER_ICON_CLASS.to_owned()} /> }
        </div>
    }
}

fn get_message_class_content_div(message_type: MessageType) -> String {
    match  message_type {
        MessageType::Incoming => MESSAGE_CONTENT_INCOMING_CLASS.to_owned(),
        MessageType::Outgoing => MESSAGE_CONTENT_OUTGOING_CLASS.to_owned()
    }
}

fn get_message_class_outer_div(message_type: MessageType) -> String {
    match message_type {
        MessageType::Outgoing => format!("{} {}", MESSAGE_CONTAINER_BASE_CLASS.to_owned(), MESSAGE_CONTAINER_OUTGOING_ADDITIONAL_CLASS.to_owned()),
        _ => MESSAGE_CONTAINER_BASE_CLASS.to_owned()
    }
}