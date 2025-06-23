use dioxus::{desktop::window, prelude::*};

use crate::{
    state::{IS_FOCUS_MODE, SMALL_SESSION_COUNT, TIMER_EXPIRED},
    ui::icons::{Icon, IconType},
};

#[component]
pub fn Info() -> Element {
    let is_focus_mode = *IS_FOCUS_MODE.read();
    let small_session_count = *SMALL_SESSION_COUNT.read();

    rsx! {
        div {
            class: "absolute top-[-5rem] left-0 w-full h-16 text-2xl font-bold space-y-2 flex flex-col items-center justify-center text-center",
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
            SessionCount {}
        }
    }
}

#[component]
fn SessionCount() -> Element {
    let mut small_session_count = use_signal(|| 0);
    let mut visible_items = use_signal(|| 0);
    let mut hidden_items = use_signal(|| 0);

    use_effect(move || {
        let session_count = *SMALL_SESSION_COUNT.read();
        small_session_count.set(session_count);

        let icon_width = 24; // 1.5rem
        let spacing = 8; // 0.5rem
        let icon_total = icon_width + spacing;

        let scale_factor = window().scale_factor();
        let window_width = window().inner_size().width / scale_factor as u32;

        let max_width = 0.3 * window_width as f32;
        let max_icons = (max_width / icon_total as f32).floor() as usize;

        let visible_item_count = session_count.min(max_icons as u32);
        visible_items.set(visible_item_count);

        let hidden_count = session_count.saturating_sub(visible_item_count);
        hidden_items.set(hidden_count);
    });

    let color = if *TIMER_EXPIRED.read() {
        "fill-red-500 stroke-red-500"
    } else if *IS_FOCUS_MODE.read() {
        "fill-blue-500 stroke-blue-500"
    } else {
        "fill-green-500 stroke-green-500"
    };

    let bg_color = if *TIMER_EXPIRED.read() {
        "bg-red-500"
    } else if *IS_FOCUS_MODE.read() {
        "bg-blue-500"
    } else {
        "bg-green-500"
    };

    let text_color = if *TIMER_EXPIRED.read() {
        "text-red-200"
    } else if *IS_FOCUS_MODE.read() {
        "text-blue-200"
    } else {
        "text-green-200"
    };

    rsx! {
        div {
            class: "flex space-x-2 items-center justify-center",
            if *hidden_items.read() > 0 {
                div {
                    class: format!("text-xs px-2 py-1 rounded-2xl {} {}", bg_color, text_color),
                    "+{hidden_items}"
                }
            }

            for i in 0..visible_items + 1 {
                Icon {
                    icon_type: if i == *visible_items.read() {
                        IconType::CircleOutlined
                    } else {
                        IconType::CircleFilled
                    },
                    size: "1.5rem",
                    opacity: 1.0,
                    class: format!("transition-all duration-200 ease-out {}", color)
                }
            }
        }
    }
}
