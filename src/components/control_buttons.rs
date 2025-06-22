use dioxus::prelude::*;

use crate::{
    components::timer::{clear_timer, next_session, revert_session, SKIPPED_SESSION},
    ui::{icon_button::IconButton, icons::IconType},
};

#[component]
pub fn ControlButtons() -> Element {
    let left_button_icon = if *SKIPPED_SESSION.read() {
        IconType::Revert
    } else {
        IconType::Restart
    };

    let left_button_action = if *SKIPPED_SESSION.read() {
        revert_session
    } else {
        clear_timer
    };

    let left_button_title = if *SKIPPED_SESSION.read() {
        "Revert session"
    } else {
        "Restart timer"
    };

    rsx! {
        div {
            class: "absolute bottom-0 left-0 flex items-end justify-center w-full h-1/5 py-4",
            IconButton {
                icon_type: left_button_icon,
                title: left_button_title,
                size: "6rem",
                action: left_button_action,
            }
            IconButton {
                icon_type: IconType::Skip,
                title: "Skip session",
                size: "6rem",
                action: next_session,
            }
        }
    }
}
