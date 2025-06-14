use dioxus::{desktop::window, html::input_data::MouseButton, prelude::*};
use objc2::{msg_send, runtime::AnyObject};

pub unsafe fn set_transparent_titlebar(ns_window: *mut AnyObject) {
    let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: true];

    let mut style_mask: u64 = msg_send![ns_window, styleMask];

    const NS_FULL_SIZE_CONTENT_VIEW_WINDOW_MASK: u64 = 1 << 15;
    style_mask |= NS_FULL_SIZE_CONTENT_VIEW_WINDOW_MASK;
    let _: () = msg_send![ns_window, setStyleMask: style_mask];

    let _: () = msg_send![ns_window, setTitleVisibility: 1i64];
    let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: true];
}

// adapted from Freya's WindowDragArea
#[component]
pub fn WindowDragArea() -> Element {
    let window = window();

    let onmousedown = move |e: MouseEvent| {
        if let Some(MouseButton::Primary) = e.data().trigger_button() {
            e.stop_propagation();
            window.drag();
        }
    };

    rsx! {
        div {
            class: "absolute top-0 left-0 w-full h-12",
            onmousedown: onmousedown,
        }
    }
}
