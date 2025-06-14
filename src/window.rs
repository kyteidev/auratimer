use objc2::{msg_send, runtime::AnyObject};

pub unsafe fn set_transparent_titlebar(ns_window: *mut AnyObject) {
    let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: true];

    let mut style_mask: u64 = msg_send![ns_window, styleMask];

    const NS_FULL_SIZE_CONTENT_VIEW_WINDOW_MASK: u64 = 1 << 15;
    style_mask |= NS_FULL_SIZE_CONTENT_VIEW_WINDOW_MASK;
    let _: () = msg_send![ns_window, setStyleMask: style_mask];

    let _: () = msg_send![ns_window, setTitleVisibility: 1i64];
    let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: true];

    let _: () = msg_send![ns_window, setMovableByWindowBackground: true];
}
