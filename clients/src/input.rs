use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

const INPUT_CONTAINER_CLASS: &str = "bg-gray-300 p-4";
const INPUT_CLASS: &str = "flex items-center h-10 w-full rounded px-3 text-sm";


#[derive(Properties, PartialEq)]
pub struct MessageInputProperties {
    pub on_message: Callback<String>,
}

#[function_component]
pub(crate) fn MessageInput(props: &MessageInputProperties) -> Html {
    let input_handle = use_state(String::default);
    let input_value = (*input_handle).clone();

    // This allows the input's html value to get updated as the user types
    // while allowing us to track it's value so we can send it later.
    let on_input_event = on_input_callback(input_handle.clone());

    // This will actually emit the message callback when the user presses enter.
    let on_input_keydown = on_keyboard_callback(input_handle.clone(), input_value.clone(), props.on_message.clone());

    html! {
        <div class = {INPUT_CONTAINER_CLASS.to_owned()}>
            <input 
                class = {INPUT_CLASS.to_owned()}
                type = "text" 
                placeholder = "Type your message…" 
                oninput = {on_input_event}
                onkeydown = {on_input_keydown}
                value = {input_value.clone()} />
        </div>
    }
}

fn on_input_callback(input_handle: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");

        let input_value = target.unchecked_into::<HtmlInputElement>().value();
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        // Here we are sure that this is input element so we can convert it to the appropriate type without checking
        input_handle.set(input_value);
    })
}

fn on_keyboard_callback(input_handle: UseStateHandle<String>, input_value: String, on_message_callback: Callback<String>) -> Callback<KeyboardEvent> {
    Callback::from(move |event: KeyboardEvent| {
        if event.code() == "Enter" {
            on_message_callback.emit(input_value.clone());
            input_handle.set(String::default())
        } else {
            ()
        }
    })
}