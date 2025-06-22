use dioxus::prelude::*;

use crate::state::{IS_FOCUS_MODE, SMALL_SESSION_COUNT};

#[component]
pub fn Info() -> Element {
    let is_focus_mode = *IS_FOCUS_MODE.read();
    let small_session_count = *SMALL_SESSION_COUNT.read();

    rsx! {
        div {
            class: "absolute top-[-4rem] left-0 w-full h-16 text-2xl font-bold flex flex-col items-center justify-center text-center",
            h1 {
                if is_focus_mode {
                    "Focus"
                } else {
                    if small_session_count % 4 == 0 && small_session_count != 0 {
                        "Long break"
                    } else {
                        "Short break"
                    }
                }
            }
        }
    }
}
