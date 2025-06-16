use dioxus::prelude::*;

use crate::components::icons::{Icon, IconType};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    icon_type: IconType,
    size: String,
    action: fn(),
}

#[component]
pub fn IconButton(props: Props) -> Element {
    let rem_pos = props.size.find("rem").unwrap();
    let number_str = props.size[..rem_pos].trim();
    let number = number_str.parse::<f64>().unwrap();

    let icon_size = number * (2.0 / 3.0);

    let size = props.size.clone();

    rsx! {
        div {
            class: "relative flex items-center justify-center",
            width: size.clone(),
            height: size.clone(),
            button {
                class: "transition duration-200 rounded-full bg-blue-500 opacity-0 hover:opacity-10 z-10 cursor-default hover:cursor-pointer",
                width: size.clone(),
                height: size,
                onclick: move |_| (props.action)(),
            }
            Icon {
                icon_type: props.icon_type,
                class: "transition duration-200 absolute fill-blue-500 stroke-blue-500",
                opacity: 1.0,
                size: format!("{}rem", icon_size),
            }
        }
    }
}
