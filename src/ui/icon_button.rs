use dioxus::prelude::*;

use crate::{
    state::{BG_COLOR_INVERTED, ICON_COLOR},
    ui::icons::{Icon, IconType},
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

    let color = *ICON_COLOR.read();
    let bg_color = *BG_COLOR_INVERTED.read();

    rsx! {
        div {
            class: "relative flex items-center justify-center",
            width: size.clone(),
            height: size.clone(),
            button {
                class: format!("transition duration-200 rounded-full opacity-0 hover:opacity-10 z-10 cursor-default hover:cursor-pointer {}", bg_color),
                title: props.title,
                width: size.clone(),
                height: size,
                onclick: move |_| (props.action)(),
            }
            Icon {
                icon_type: props.icon_type,
                class: format!("transition duration-200 absolute {}", color),
                opacity: 1.0,
                size: format!("{}rem", icon_size),
            }
        }
    }
}
