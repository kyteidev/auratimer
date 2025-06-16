use dioxus::{
    desktop::{tao::platform::macos::WindowExtMacOS, window},
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
    ui::icon_button::IconButton,
    window::{set_transparent_titlebar, WindowDragArea},
};

mod components;
mod ui;
mod window;

fn main() {
    let _ = FmtSubscriber::builder().init();

    dioxus::LaunchBuilder::desktop().launch(App);
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

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {
            class: "bg-blue-200 text-blue-500 w-screen h-screen select-none flex flex-col",
            WindowDragArea {}
            div {
                class: "flex-grow flex items-center justify-center",
                Timer {}
            }
            div {
                class: "absolute bottom-0 left-0 flex items-center justify-center w-full h-1/5 bg-",
                IconButton {
                    icon_type: IconType::Restart,
                    size: "6rem",
                    action: clear_timer,
                }
            }
        }
    }
}
