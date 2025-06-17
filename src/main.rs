use dioxus::{
    desktop::{tao::platform::macos::WindowExtMacOS, window, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use objc2::msg_send;
use tracing::error;
use tracing_subscriber::FmtSubscriber;

use crate::{
    components::{
        icons::IconType,
        timer::{clear_timer, Timer},
    },
    state::TIMER_EXPIRED,
    ui::icon_button::IconButton,
    window::{set_transparent_titlebar, WindowDragArea},
};

mod components;
mod state;
mod ui;
mod window;

fn main() {
    let _ = FmtSubscriber::builder().init();

    let config = Config::new()
        .with_window(WindowBuilder::new().with_inner_size(LogicalSize::new(900.0, 600.0)));

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let ns_view: *mut objc2::runtime::AnyObject = window().ns_view().cast();
    unsafe {
        let ns_window: *mut objc2::runtime::AnyObject = msg_send![ns_view, window];
        if ns_window.is_null() {
            error!("ns_window is null, unable to set transparent titlebar");
        }
        set_transparent_titlebar(ns_window);
    }

    let timer_expired = *TIMER_EXPIRED.read();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {
            class: format!("w-screen h-screen select-none flex flex-col {}", if timer_expired { "bg-red-200 text-red-500" } else { "bg-blue-200 text-blue-500" }),
            WindowDragArea {}
            div {
                class: "flex-grow flex items-center justify-center",
                Timer {}
            }
            div {
                class: "absolute bottom-0 left-0 flex items-end justify-center w-full h-1/5 py-4",
                IconButton {
                    icon_type: IconType::Restart,
                    size: "6rem",
                    action: clear_timer,
                }
            }
        }
    }
}
