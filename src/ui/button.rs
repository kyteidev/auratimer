use dioxus::prelude::*;

use crate::state::{IS_FOCUS_MODE, TIMER_EXPIRED};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    action: fn(),
    title: String,
    class: String,
    text: String,
}

#[component]
pub fn Button(props: Props) -> Element {
    let bg_color = if *TIMER_EXPIRED.read() {
        "bg-red-500"
    } else if *IS_FOCUS_MODE.read() {
        "bg-blue-500"
    } else {
        "bg-green-500"
    };

    rsx! {
        button {
            class: format!("transition duration-200 rounded-2xl z-10 cursor-default hover:cursor-pointer {} {}", bg_color, props.class),
            title: props.title,
            onclick: move |_| (props.action)(),
            {props.text}
        }
    }
}
