use dioxus::prelude::*;

use crate::state::BG_COLOR_INVERTED;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    action: fn(),
    title: String,
    class: String,
    text: String,
}

#[component]
pub fn Button(props: Props) -> Element {
    let bg_color = *BG_COLOR_INVERTED.read();

    rsx! {
        button {
            class: format!("transition duration-200 rounded-2xl z-10 cursor-default hover:cursor-pointer {} {}", bg_color, props.class),
            title: props.title,
            onclick: move |_| (props.action)(),
            {props.text}
        }
    }
}
