use std::time::Duration;

use dioxus::prelude::*;
use tokio::time::Instant;

use crate::{
    components::icons::{Icon, IconType},
    sound::play_alarm,
    state::{IS_FOCUS_MODE, TIMER_EXPIRED},
    tray::set_tray_title,
};

const FOCUS_DURATION: u32 = 25 * 60 * 1000;
const BREAK_DURATION: u32 = 5 * 60 * 1000;

static TIMER_AMOUNT: GlobalSignal<u32> = GlobalSignal::new(|| {
    if *IS_FOCUS_MODE.read() {
        FOCUS_DURATION
    } else {
        BREAK_DURATION
    }
});

static TIMER_RUNNING: GlobalSignal<bool> = GlobalSignal::new(|| false);
static MILLIS_REMAINING: GlobalSignal<u32> = GlobalSignal::new(|| *TIMER_AMOUNT.read());

pub fn clear_timer() {
    *TIMER_RUNNING.write() = false;
    *MILLIS_REMAINING.write() = *TIMER_AMOUNT.read();
    *TIMER_EXPIRED.write() = false;
}

#[component]
pub fn Timer() -> Element {
    let mut hovering = use_signal(|| false);
    let mut start_time = use_signal(|| None::<Instant>);
    let mut last_seconds = use_signal(|| None::<u32>);

    let mut formatted_time = use_signal(String::new);

    use_effect(move || {
        formatted_time.set({
            let minutes = *MILLIS_REMAINING.read() / 1000 / 60;
            let seconds = *MILLIS_REMAINING.read() / 1000 % 60;

            // Only update tray title if seconds actually changed for performance
            if *last_seconds.peek() != Some(seconds) {
                set_tray_title(format!("[{:02}:{:02}]", minutes, seconds).as_str());
                last_seconds.set(Some(seconds));
            }

            format!("{:02}:{:02}", minutes, seconds)
        });
    });

    let toggle_timer = move |_| {
        if !*TIMER_RUNNING.read() {
            if *MILLIS_REMAINING.read() == 0 {
                *MILLIS_REMAINING.write() = *TIMER_AMOUNT.read();
                *TIMER_EXPIRED.write() = false;
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

                    if remaining_time > 0 {
                        loop {
                            interval.tick().await;
                            if !*TIMER_RUNNING.peek() {
                                break;
                            }
                            let elapsed_time = timer_start.elapsed();
                            let remaining_time = Duration::from_millis(remaining_time as u64)
                                .saturating_sub(elapsed_time)
                                .as_millis();
                            *MILLIS_REMAINING.write() = remaining_time as u32;
                            if remaining_time == 0 {
                                *TIMER_EXPIRED.write() = true;
                                *TIMER_RUNNING.write() = false;

                                let is_focus_mode = *IS_FOCUS_MODE.peek();
                                *IS_FOCUS_MODE.write() = !is_focus_mode;

                                play_alarm();
                                break;
                            }
                        }
                    }
                });
            }
        }
    });

    let opacity = if *hovering.read() { 0.1 } else { 1.0 };

    let color = if *TIMER_EXPIRED.read() {
        "hover:bg-red-500"
    } else if *IS_FOCUS_MODE.read() {
        "hover:bg-blue-500"
    } else {
        "hover:bg-green-500"
    };

    rsx! {
        div {
            class: "relative bg-transparent w-3/5 h-2/5 rounded-lg text-[10rem] flex items-center justify-center",
            div {
                class: format!("transition duration-200 absolute top-0 left-0 w-full h-full opacity-10 rounded-lg bg-transparent z-10 cursor-pointer flex items-center justify-center {}", color),
                title: "Toggle timer",
                onclick: toggle_timer,
                onmouseenter: move |_| hovering.set(true),
                onmouseleave: move |_| hovering.set(false),
            }
            Icon {
                icon_type: if *TIMER_RUNNING.read() { IconType::Pause } else { IconType::Start },
                class: format!("transition duration-200 absolute"),
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
