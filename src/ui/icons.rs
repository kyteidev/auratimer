// SVG icons from Tabler Icons (https://tabler.io/icons). SVG icons are licensed under the MIT License.

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum IconType {
    Start,
    Pause,
    Restart,
    Skip,
    Revert,
    CircleFilled,
    CircleOutlined,
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
                d: "M6 4v16a1 1 0 0 0 1.524 .852l13 -8a1 1 0 0 0 0 -1.704l-13 -8a1 1 0 0 0 -1.524 .852z",
            }
        ),
        IconType::Pause => rsx!(
            path {
                d: "M9 4h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2z",
            }
            path {
                d: "M17 4h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2z",
            }
        ),
        IconType::Restart => rsx!(
            path {
                d: "M19.95 11a8 8 0 1 0 -.5 4m.5 5v-5h-5",
                fill: "none"
            }
        ),
        IconType::Skip => rsx!(
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
                stroke: "none"
            }
            path {
                d: "M4 5v14l12 -7z",
                fill: "none",
            }
            path {
                d: "M20 5l0 14",
                fill: "none",
            }
        ),
        IconType::Revert => rsx!(
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
                stroke: "none"
            }
            path {
                d: "M9 14l-4 -4l4 -4",
                fill: "none",
            }
            path {
                d: "M5 10h11a4 4 0 1 1 0 8h-1",
                fill: "none",
            }
        ),
        IconType::CircleFilled => rsx!(
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
                stroke: "none"
            }
            path {
                d: "M7 3.34a10 10 0 1 1 -4.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 4.995 -8.336z"
            }
        ),
        IconType::CircleOutlined => rsx!(
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
                stroke: "none"
            }
            path {
                d: "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0",
                fill: "none",
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
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            {children}
        }
    )
}
