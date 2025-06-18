use dioxus::prelude::*;

use crate::{
    components::icons::{Icon, IconType},
    state::TIMER_EXPIRED,
};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    icon_type: IconType,
    size: String,
    action: fn(),
    title: String,
}

#[component]
pub fn IconButton(props: Props) -> Element {
    let rem_pos = props.size.find("rem").unwrap();
    let number_str = props.size[..rem_pos].trim();
    let number = number_str.parse::<f64>().unwrap();

    let icon_size = number * (2.0 / 3.0);

    let size = props.size.clone();

    let timer_expired = *TIMER_EXPIRED.read();

    rsx! {
        div {
            class: "relative flex items-center justify-center",
            width: size.clone(),
            height: size.clone(),
            button {
                class: format!("transition duration-200 rounded-full opacity-0 hover:opacity-10 z-10 cursor-default hover:cursor-pointer {}", if timer_expired { "bg-red-500" } else { "bg-blue-500" }),
                title: props.title,
                width: size.clone(),
                height: size,
                onclick: move |_| (props.action)(),
            }
            Icon {
                icon_type: props.icon_type,
                class: format!("transition duration-200 absolute {}", if timer_expired { "fill-red-500 stroke-red-500" } else { "fill-blue-500 stroke-blue-500" }),
                opacity: 1.0,
                size: format!("{}rem", icon_size),
            }
        }
    }
}
