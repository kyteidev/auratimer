use dioxus::prelude::*;

use crate::components::icons::{Icon, IconType};

#[component]
pub fn Timer() -> Element {
    let mut timer_running = use_signal(|| false);
    let mut seconds_remaining = use_signal(|| 25 * 60);

    let mut hovering = use_signal(|| false);

    let formatted_time = {
        let minutes = *seconds_remaining.read() / 60;
        let seconds = *seconds_remaining.read() % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    let toggle_timer = move |_| {
        if !*timer_running.read() {
            if *seconds_remaining.read() == 0 {
                seconds_remaining.set(25 * 60);
            }
            timer_running.set(true);
        } else {
            timer_running.set(false);
        }
    };

    use_effect(move || {
        if *timer_running.read() {
            spawn(async move {
                let interval = std::time::Duration::from_millis(1000);
                let mut interval = tokio::time::interval(interval);

                while *seconds_remaining.read() > 0 {
                    interval.tick().await;
                    if !*timer_running.read() {
                        break;
                    }
                    let current = *seconds_remaining.read();
                    seconds_remaining.set(current - 1);
                }

                if *seconds_remaining.read() == 0 {
                    timer_running.set(false);
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
                class: "transition duration-100 absolute top-0 left-0 w-full h-full opacity-10 rounded-lg bg-transparent hover:bg-gray-500 z-10 cursor-pointer flex items-center justify-center",
                onclick: toggle_timer,
                onmouseenter: move |_| hovering.set(true),
                onmouseleave: move |_| hovering.set(false),
            }
            Icon {
                icon_type: if *timer_running.read() { IconType::Pause } else { IconType::Start },
                class: "transition duration-100 absolute",
                opacity: if *hovering.read() { 1.0 } else { 0.0 },
                color: "#000000",
                size: "96px",
            }
            p {
                class: "transition duration-100",
                opacity: "{opacity}",
                "{formatted_time}"
            }
        }
    }
}
