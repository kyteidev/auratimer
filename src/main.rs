mod sound;

use std::sync::mpsc::channel;

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
        control_buttons::ControlButtons, info::Info, timer::Timer, timer_expired::TimerExpired,
    },
    state::{init_colors, BG_COLOR, TEXT_COLOR, TIMER_EXPIRED},
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
        let (tx, rx) = channel::<TrayIconEvent>();
        *TRAY_EVENT_SENDER.lock().unwrap() = Some(tx);
        *TRAY_EVENT_RECEIVER.lock().unwrap() = Some(rx);

        let (window_tx, window_rx) = channel();
        *WINDOW_COMMAND_SENDER.lock().unwrap() = Some(window_tx);
        *WINDOW_COMMAND_RECEIVER.lock().unwrap() = Some(window_rx);

        init_tray();
        init_tray_handler();
        init_tray_listener();

        let ns_view: *mut objc2::runtime::AnyObject = window().ns_view().cast();
        unsafe {
            let ns_window: *mut objc2::runtime::AnyObject = msg_send![ns_view, window];
            if ns_window.is_null() {
                error!("ns_window is null, unable to set transparent titlebar");
            }
            set_transparent_titlebar(ns_window);
        }
    });

    use_future(move || async move {
        loop {
            handle_window_commands();
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
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
