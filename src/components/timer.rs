use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;

use crate::components::icons::{Icon, IconType};

const DURATION: u32 = 25 * 60 * 1000;

static TIMER_RUNNING: GlobalSignal<bool> = GlobalSignal::new(|| false);
static MILLIS_REMAINING: GlobalSignal<u32> = GlobalSignal::new(|| DURATION);

pub fn clear_timer() {
    *TIMER_RUNNING.write() = false;
    *MILLIS_REMAINING.write() = DURATION;
}

#[component]
pub fn Timer() -> Element {
    let mut hovering = use_signal(|| false);
    let mut start_time = use_signal(|| 0);

    let formatted_time = {
        let minutes = *MILLIS_REMAINING.read() / 1000 / 60;
        let seconds = *MILLIS_REMAINING.read() / 1000 % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    fn get_current_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    let toggle_timer = move |_| {
        if !*TIMER_RUNNING.read() {
            if *MILLIS_REMAINING.read() == 0 {
                *MILLIS_REMAINING.write() = DURATION;
            }
            *TIMER_RUNNING.write() = true;
            *start_time.write() = get_current_time();
        } else {
            *TIMER_RUNNING.write() = false;
        }
    };

    use_effect(move || {
        if *TIMER_RUNNING.read() {
            spawn(async move {
                let interval = std::time::Duration::from_millis(100); // calculate time every 100ms
                let mut interval = tokio::time::interval(interval);

                let remaining_time = *MILLIS_REMAINING.read();

                while *MILLIS_REMAINING.read() > 0 {
                    interval.tick().await;
                    if !*TIMER_RUNNING.read() {
                        break;
                    }
                    let elapsed_time = get_current_time() - *start_time.peek();
                    let remaining_time = remaining_time - elapsed_time as u32;
                    *MILLIS_REMAINING.write() = remaining_time;
                }
                *TIMER_RUNNING.write() = false;
            });
        }
    });

    let opacity = if *hovering.read() { 0.1 } else { 1.0 };

    rsx! {
        div {
            class: "relative bg-transparent w-3/5 h-2/5 rounded-lg text-[10rem] flex items-center justify-center",
            div {
                class: "transition duration-200 absolute top-0 left-0 w-full h-full opacity-10 rounded-lg bg-transparent hover:bg-blue-500 z-10 cursor-pointer flex items-center justify-center",
                onclick: toggle_timer,
                onmouseenter: move |_| hovering.set(true),
                onmouseleave: move |_| hovering.set(false),
            }
            Icon {
                icon_type: if *TIMER_RUNNING.read() { IconType::Pause } else { IconType::Start },
                class: "transition duration-200 absolute fill-blue-500",
                opacity: if *hovering.read() { 1.0 } else { 0.0 },
                size: "96px",
            }
            p {
                class: "transition duration-200",
                opacity: "{opacity}",
                "{formatted_time}"
            }
        }
    }
}
