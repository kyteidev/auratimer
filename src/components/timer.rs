use std::time::Duration;

use core_graphics::display::{CGDisplayBounds, CGGetActiveDisplayList, CGPoint};
use dioxus::{
    desktop::{window, Config, LogicalPosition, WindowBuilder},
    prelude::*,
};
use tokio::time::Instant;
use tracing::error;
use tray_icon::dpi::LogicalSize;

use crate::{
    components::alert::Alert,
    sound::play_alarm,
    state::{
        BG_COLOR_HOVER, FULL_SESSION_COUNT, ICON_COLOR, IS_FOCUS_MODE, SMALL_SESSION_COUNT,
        TIMER_EXPIRED,
    },
    tray::set_tray_title,
    ui::icons::{Icon, IconType},
};

const FOCUS_DURATION: u32 = 25 * 60 * 1000;
const SHORT_BREAK_DURATION: u32 = 5 * 60 * 1000;
const LONG_BREAK_DURATION: u32 = 20 * 60 * 1000;

static TIMER_RUNNING: GlobalSignal<bool> = GlobalSignal::new(|| false);
static MILLIS_REMAINING: GlobalSignal<u32> = GlobalSignal::new(|| FOCUS_DURATION);

static LAST_SAVED_TIME: GlobalSignal<u32> = GlobalSignal::new(|| 0);
pub static SKIPPED_SESSION: GlobalSignal<bool> = GlobalSignal::new(|| false);

static START_TIME: GlobalSignal<Option<Instant>> = GlobalSignal::new(|| None);

pub fn clear_timer() {
    let small_session_count = *SMALL_SESSION_COUNT.peek();

    *TIMER_RUNNING.write() = false;
    *MILLIS_REMAINING.write() = if *IS_FOCUS_MODE.peek() {
        FOCUS_DURATION
    } else if small_session_count % 4 == 0 && small_session_count != 0 {
        LONG_BREAK_DURATION
    } else {
        SHORT_BREAK_DURATION
    };
    *TIMER_EXPIRED.write() = false;
}

pub fn start_timer() {
    clear_timer();

    *START_TIME.write() = Some(Instant::now());
    *TIMER_RUNNING.write() = true;
}

pub fn next_session() {
    let is_focus_mode = *IS_FOCUS_MODE.peek();
    *IS_FOCUS_MODE.write() = !is_focus_mode;

    *LAST_SAVED_TIME.write() = *MILLIS_REMAINING.read();
    *SKIPPED_SESSION.write() = true;

    let small_session_count = *SMALL_SESSION_COUNT.peek();
    // used remainder for session stats
    if small_session_count % 4 == 0 && small_session_count != 0 {
        *FULL_SESSION_COUNT.write() += 1;
    }
    if !is_focus_mode {
        *SMALL_SESSION_COUNT.write() += 1;
    }

    clear_timer();

    let minutes = *MILLIS_REMAINING.peek() / 1000 / 60;
    let seconds = *MILLIS_REMAINING.peek() / 1000 % 60;
    update_tray(minutes, seconds);
}

pub fn revert_session() {
    let is_focus_mode = *IS_FOCUS_MODE.peek();

    if is_focus_mode {
        *SMALL_SESSION_COUNT.write() -= 1;
    }

    *IS_FOCUS_MODE.write() = !is_focus_mode;

    *TIMER_RUNNING.write() = false;
    *MILLIS_REMAINING.write() = *LAST_SAVED_TIME.peek();

    *SKIPPED_SESSION.write() = false;

    let minutes = *MILLIS_REMAINING.peek() / 1000 / 60;
    let seconds = *MILLIS_REMAINING.peek() / 1000 % 60;
    update_tray(minutes, seconds);
}

#[component]
pub fn Timer() -> Element {
    let mut hovering = use_signal(|| false);
    let mut last_seconds = use_signal(|| None::<u32>);

    let mut formatted_time = use_signal(String::new);

    use_effect(move || {
        formatted_time.set({
            let minutes = *MILLIS_REMAINING.read() / 1000 / 60;
            let seconds = *MILLIS_REMAINING.read() / 1000 % 60;

            // Only update tray title if seconds actually changed for performance
            if *last_seconds.peek() != Some(seconds) {
                update_tray(minutes, seconds);
                last_seconds.set(Some(seconds));
            }

            format!("{:02}:{:02}", minutes, seconds)
        });
    });

    let toggle_timer = move |_| {
        if !*TIMER_RUNNING.read() {
            *START_TIME.write() = Some(Instant::now());
            *TIMER_RUNNING.write() = true;

            *SKIPPED_SESSION.write() = false;
        } else {
            *TIMER_RUNNING.write() = false;
        }
    };

    use_effect(move || {
        if *TIMER_RUNNING.read() {
            if let Some(timer_start) = *START_TIME.peek() {
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

                                let small_session_count = *SMALL_SESSION_COUNT.peek();
                                // used remainder for session stats
                                if small_session_count % 4 == 0 && small_session_count != 0 {
                                    *FULL_SESSION_COUNT.write() += 1;
                                }

                                if !is_focus_mode {
                                    *SMALL_SESSION_COUNT.write() += 1;

                                    set_tray_title("Focus time!");
                                } else {
                                    set_tray_title("Break time!");
                                }

                                *IS_FOCUS_MODE.write() = !is_focus_mode;

                                show_alert_window();

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
    let color = *BG_COLOR_HOVER.read();
    let icon_color = *ICON_COLOR.read();

    rsx! {
        div {
            class: "relative bg-transparent w-4/7 rounded-lg text-[10rem] flex items-center justify-center",
            style: "height: calc(100vh * 0.33",
            div {
                class: format!("transition duration-200 absolute top-0 left-0 w-full h-full opacity-10 rounded-lg bg-transparent z-10 cursor-pointer flex items-center justify-center {}", color),
                title: "Toggle timer",
                onclick: toggle_timer,
                onmouseenter: move |_| hovering.set(true),
                onmouseleave: move |_| hovering.set(false),
            }
            Icon {
                icon_type: if *TIMER_RUNNING.read() { IconType::Pause } else { IconType::Start },
                class: format!("transition duration-200 absolute stroke-none {}", icon_color),
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

fn update_tray(minutes: u32, seconds: u32) {
    let session_type = if *IS_FOCUS_MODE.read() {
        "Focus"
    } else {
        "Break"
    };
    set_tray_title(format!("{}: {:02}:{:02}", session_type, minutes, seconds).as_str());
}

fn show_alert_window() {
    let dom = VirtualDom::new(Alert);

    let width = 600.0;
    let height = 64.0;

    let (active_display_id, display_width, _) = get_active_display_and_size();

    let display_bounds = unsafe { CGDisplayBounds(active_display_id as u32) };

    let x_pos = display_bounds.origin.x + (display_width / 2.0 - width / 2.0);
    let y_pos = 64.0;

    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(width, height))
                .with_position(LogicalPosition::new(x_pos, y_pos))
                .with_closable(false)
                .with_always_on_top(true)
                .with_transparent(true)
                .with_resizable(false)
                .with_decorations(false),
        )
        .with_disable_context_menu(true);
    window().new_window(dom, config);
}

extern "C" {
    fn CGEventCreate(source: *const std::ffi::c_void) -> *mut std::ffi::c_void;
    fn CGEventGetLocation(event: *mut std::ffi::c_void) -> CGPoint;
    fn CFRelease(cf: *mut std::ffi::c_void);
}

fn get_active_display_and_size() -> (i32, f64, f64) {
    let mouse_location = unsafe {
        let event = CGEventCreate(std::ptr::null());
        let loc = CGEventGetLocation(event);
        CFRelease(event);
        loc
    };

    // Get all active displays
    let mut display_ids = [0u32; 16];
    let mut display_count = 0;
    unsafe {
        CGGetActiveDisplayList(
            display_ids.len() as u32,
            display_ids.as_mut_ptr(),
            &mut display_count,
        );
    }

    for &display_id in &display_ids[..display_count as usize] {
        let bounds = unsafe { CGDisplayBounds(display_id) };
        if point_in_rect(mouse_location.x, mouse_location.y, &bounds) {
            return (display_id as i32, bounds.size.width, bounds.size.height);
        }
    }
    error!("Cursor is not on any detected display");

    (1, 0.0, 0.0)
}

fn point_in_rect(x: f64, y: f64, rect: &core_graphics::geometry::CGRect) -> bool {
    x >= rect.origin.x
        && x < rect.origin.x + rect.size.width
        && y >= rect.origin.y
        && y < rect.origin.y + rect.size.height
}
