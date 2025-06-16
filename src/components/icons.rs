// SVG icons from Tabler Icons (https://tabler.io/icons). SVG icons are licensed under the MIT License.

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum IconType {
    Start,
    Pause,
}

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    icon_type: IconType,
    class: String,
    opacity: f32,
    size: String,
}

#[component]
pub fn Icon(props: Props) -> Element {
    let children = match props.icon_type {
        IconType::Start => rsx!(
            path {
                d: "M6 4v16a1 1 0 0 0 1.524 .852l13 -8a1 1 0 0 0 0 -1.704l-13 -8a1 1 0 0 0 -1.524 .852z"
            }
        ),
        IconType::Pause => rsx!(
            path {
                d: "M9 4h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2z"
            }
            path {
                d: "M17 4h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2z"
            }
        ),
    };

    rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            class: "{props.class}",
            width: "{props.size}",
            height: "{props.size}",
            opacity: "{props.opacity}",
            path { d: "M0 0h24v24H0z", fill: "none" }
            {children}
        }
    )
}
