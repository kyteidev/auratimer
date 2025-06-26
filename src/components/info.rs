use dioxus::{desktop::window, prelude::*};

use crate::{
    state::{
        BG_COLOR_INVERTED, ICON_COLOR, IS_FOCUS_MODE, SMALL_SESSION_COUNT, TEXT_COLOR_INVERTED,
    },
    ui::icons::{Icon, IconType},
};

static VISIBLE_ITEMS: GlobalSignal<u32> = GlobalSignal::new(|| 0);
static HIDDEN_ITEMS: GlobalSignal<u32> = GlobalSignal::new(|| 0);

#[component]
pub fn Info() -> Element {
    let is_focus_mode = *IS_FOCUS_MODE.read();
    let small_session_count = *SMALL_SESSION_COUNT.read();

    fn update_visible_sessions(small_session_count: u32) {
        let icon_width = 24; // 1.5rem
        let spacing = 8; // 0.5rem
        let icon_total = icon_width + spacing;

        let scale_factor = window().scale_factor();
        let window_width = window().outer_size().width / scale_factor as u32;

        let max_width = 0.3 * window_width as f32;
        let max_icons = (max_width / icon_total as f32).floor() as usize;

        let visible_item_count = small_session_count.min(max_icons as u32);
        *VISIBLE_ITEMS.write() = visible_item_count;

        let hidden_count = small_session_count.saturating_sub(visible_item_count);
        *HIDDEN_ITEMS.write() = hidden_count;
    }

    use_effect(move || {
        let small_session_count = *SMALL_SESSION_COUNT.read();
        update_visible_sessions(small_session_count);
    });

    rsx! {
        div {
            class: "absolute top-[-5rem] left-0 w-full h-16 text-2xl font-bold space-y-2 flex flex-col items-center justify-center text-center",
            onresize: move |_| {
                update_visible_sessions(small_session_count);
            },
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
    let color = *ICON_COLOR.read();
    let bg_color = *BG_COLOR_INVERTED.read();
    let text_color = *TEXT_COLOR_INVERTED.read();

    rsx! {
        div {
            class: "flex space-x-2 items-center justify-center",
            if *HIDDEN_ITEMS.read() > 0 {
                div {
                    class: format!("text-xs px-2 py-1 rounded-2xl text-center flex justify-center items-center {} {}", bg_color, text_color),
                    "+{HIDDEN_ITEMS}"
                }
            }

            for i in 0..*VISIBLE_ITEMS.read() + 1 {
                Icon {
                    icon_type: if i == *VISIBLE_ITEMS.read() {
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
