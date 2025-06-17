use std::time::Duration;

use dioxus::prelude::*;
use tokio::time::Instant;

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
    let mut start_time = use_signal(|| None::<Instant>);

    let formatted_time = {
        let minutes = *MILLIS_REMAINING.read() / 1000 / 60;
        let seconds = *MILLIS_REMAINING.read() / 1000 % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };
    let toggle_timer = move |_| {
        if !*TIMER_RUNNING.read() {
            if *MILLIS_REMAINING.read() == 0 {
                *MILLIS_REMAINING.write() = DURATION;
            }
            start_time.set(Some(Instant::now()));
            *TIMER_RUNNING.write() = true;
        } else {
            *TIMER_RUNNING.write() = false;
        }
    };

    use_effect(move || {
        if *TIMER_RUNNING.read() {
            if let Some(timer_start) = *start_time.peek() {
                spawn(async move {
                    let interval = Duration::from_millis(100); // update timer every 100ms for accuracy
                    let mut interval = tokio::time::interval(interval);

                    let remaining_time = *MILLIS_REMAINING.peek();

                    while *MILLIS_REMAINING.peek() > 0 {
                        interval.tick().await;
                        if !*TIMER_RUNNING.peek() {
                            break;
                        }
                        let elapsed_time = timer_start.elapsed();
                        let remaining_time = Duration::from_millis(remaining_time as u64)
                            .saturating_sub(elapsed_time)
                            .as_millis();
                        *MILLIS_REMAINING.write() = remaining_time as u32;
                    }
                    *TIMER_RUNNING.write() = false;
                });
            }
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
