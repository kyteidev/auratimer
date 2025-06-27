mod sound;

use std::{sync::mpsc::channel, time::Duration};

use dioxus::{
    desktop::{
        tao::platform::macos::WindowExtMacOS, window, Config, LogicalSize, WindowBuilder,
        WindowCloseBehaviour,
    },
    prelude::*,
};
use objc2::msg_send;
use tracing::error;
use tracing_subscriber::FmtSubscriber;
use tray_icon::TrayIconEvent;

use crate::{
    components::{
        alert::{
            handle_timer_commands, init_timer_event_listener, TIMER_COMMAND_RECEIVER,
            TIMER_COMMAND_SENDER, TIMER_EVENT_RECEIVER, TIMER_EVENT_SENDER,
        },
        control_buttons::ControlButtons,
        info::Info,
        timer::Timer,
        timer_expired::TimerExpired,
    },
    state::{
        init_colors, BG_COLOR, IS_FOCUS_MODE, IS_FOCUS_MODE_MUTEX, SMALL_SESSION_COUNT,
        SMALL_SESSION_COUNT_MUTEX, TEXT_COLOR, TIMER_EXPIRED,
    },
    tray::{
        handle_window_commands, init_tray, init_tray_handler, init_tray_listener,
        TRAY_EVENT_RECEIVER, TRAY_EVENT_SENDER, WINDOW_COMMAND_RECEIVER, WINDOW_COMMAND_SENDER,
    },
    window::{set_transparent_titlebar, WindowDragArea},
};

mod components;
mod state;
mod tray;
mod ui;
mod window;

fn main() {
    FmtSubscriber::builder().init();

    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(900.0, 600.0))
                .with_min_inner_size(LogicalSize::new(400.0, 300.0)),
        )
        .with_close_behaviour(WindowCloseBehaviour::LastWindowHides);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    init_colors();

    use_hook(|| {
        init_channels();
        init_tray();
        init_tray_handler();
        init_tray_listener();
        setup_window();
        init_timer_event_listener();
    });

    use_future(move || async move {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        loop {
            interval.tick().await;
            handle_window_commands();
            handle_timer_commands();
        }
    });

    use_effect(|| {
        *IS_FOCUS_MODE_MUTEX.lock().unwrap() = *IS_FOCUS_MODE.read();
        *SMALL_SESSION_COUNT_MUTEX.lock().unwrap() = *SMALL_SESSION_COUNT.read();
    });

    let bg_color = *BG_COLOR.read();
    let text_color = *TEXT_COLOR.read();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {
            class: format!("w-screen h-screen select-none flex flex-col {} {}", bg_color, text_color),
            WindowDragArea {}
            if *TIMER_EXPIRED.read() {
                div {
                    class: "flex-grow flex items-center justify-center",
                    TimerExpired {}
                }
            } else {
                div {
                    class: "flex-grow flex items-center justify-center",
                    div {
                        class: "relative flex items-center justify-center w-full",
                        Info {}
                        Timer {}
                    }
                }
                ControlButtons {}
            }
        }
    }
}

fn init_channels() {
    // Tray channels
    let (tray_tx, tray_rx) = channel::<TrayIconEvent>();
    *TRAY_EVENT_SENDER.lock().unwrap() = Some(tray_tx);
    *TRAY_EVENT_RECEIVER.lock().unwrap() = Some(tray_rx);

    // Window command channels
    let (window_tx, window_rx) = channel();
    *WINDOW_COMMAND_SENDER.lock().unwrap() = Some(window_tx);
    *WINDOW_COMMAND_RECEIVER.lock().unwrap() = Some(window_rx);

    // Timer channels
    let (timer_command_tx, timer_command_rx) = channel();
    *TIMER_COMMAND_SENDER.lock().unwrap() = Some(timer_command_tx);
    *TIMER_COMMAND_RECEIVER.lock().unwrap() = Some(timer_command_rx);

    let (timer_event_tx, timer_event_rx) = channel();
    *TIMER_EVENT_SENDER.lock().unwrap() = Some(timer_event_tx);
    *TIMER_EVENT_RECEIVER.lock().unwrap() = Some(timer_event_rx);
}

fn setup_window() {
    let ns_view: *mut objc2::runtime::AnyObject = window().ns_view().cast();
    unsafe {
        let ns_window: *mut objc2::runtime::AnyObject = msg_send![ns_view, window];
        if ns_window.is_null() {
            error!("ns_window is null, unable to set transparent titlebar");
            return;
        }
        set_transparent_titlebar(ns_window);
    }
}
