use dioxus::prelude::*;

use crate::components::icons::{Icon, IconType};

static TIMER_RUNNING: GlobalSignal<bool> = GlobalSignal::new(|| false);
static SECONDS_REMAINING: GlobalSignal<u32> = GlobalSignal::new(|| 25 * 60);

pub fn clear_timer() {
    *TIMER_RUNNING.write() = false;
    *SECONDS_REMAINING.write() = 25 * 60;
}

#[component]
pub fn Timer() -> Element {
    let mut hovering = use_signal(|| false);

    let formatted_time = {
        let minutes = *SECONDS_REMAINING.read() / 60;
        let seconds = *SECONDS_REMAINING.read() % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    let toggle_timer = move |_| {
        if !*TIMER_RUNNING.read() {
            if *SECONDS_REMAINING.read() == 0 {
                *SECONDS_REMAINING.write() = 25 * 60;
            }
            *TIMER_RUNNING.write() = true;
        } else {
            *TIMER_RUNNING.write() = false;
        }
    };

    use_effect(move || {
        if *TIMER_RUNNING.read() {
            spawn(async move {
                let interval = std::time::Duration::from_millis(1000);
                let mut interval = tokio::time::interval(interval);

                while *SECONDS_REMAINING.read() > 0 {
                    interval.tick().await;
                    if !*TIMER_RUNNING.read() {
                        break;
                    }
                    let current = *SECONDS_REMAINING.read();
                    *SECONDS_REMAINING.write() = current - 1;
                }

                if *SECONDS_REMAINING.read() == 0 {
                    *TIMER_RUNNING.write() = false;
                }
            });
        }
        ()
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
