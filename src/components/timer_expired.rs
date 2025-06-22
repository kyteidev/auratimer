use dioxus::prelude::*;

use crate::{
    components::timer::start_timer,
    state::{IS_FOCUS_MODE, SMALL_SESSION_COUNT},
    ui::button::Button,
};

#[component]
pub fn TimerExpired() -> Element {
    let is_focus_mode = *IS_FOCUS_MODE.read();
    let small_session_count = *SMALL_SESSION_COUNT.peek();

    rsx! {
        div {
            class: "text-6xl font-bold flex flex-col items-center justify-center text-center space-y-8",
            h1 {
                if is_focus_mode {
                    "It's time to focus!"
                } else {
                    if small_session_count % 4 == 0 && small_session_count != 0 {
                        "It's time for your long break!"
                    } else {
                        "It's time for your short break!"
                    }
                }
            }
            Button {
                title: "Start timer",
                action: start_timer,
                class: "w-32 h-12 text-xl text-red-200",
                text: if is_focus_mode {
                    "Start focus"
                } else {
                    "Start break"
                }
            }
        }
    }
}
