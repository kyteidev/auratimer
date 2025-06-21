use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    thread,
};

use tracing::error;
use tray_icon::{TrayIconBuilder, TrayIconEvent};

pub static TRAY_EVENT_SENDER: Mutex<Option<Sender<TrayIconEvent>>> = Mutex::new(None);
pub static TRAY_EVENT_RECEIVER: Mutex<Option<Receiver<TrayIconEvent>>> = Mutex::new(None);

thread_local! {
    static TRAY_ICON: Mutex<Option<tray_icon::TrayIcon>> = Mutex::new(None);
}

pub fn init_tray_handler() {
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        if let Some(sender) = TRAY_EVENT_SENDER.lock().unwrap().as_ref() {
            let _ = sender.send(event);
        }
    }));
}

pub fn init_tray_listener() {
    thread::spawn(move || {
        loop {
            if let Some(receiver) = TRAY_EVENT_RECEIVER.lock().unwrap().as_ref() {
                match receiver.recv() {
                    Ok(tray_event) => {
                        if let TrayIconEvent::Click {
                            button: tray_icon::MouseButton::Left,
                            button_state: tray_icon::MouseButtonState::Up,
                            ..
                        } = tray_event
                        {
                            // TODO: show app window if it's hidden
                        }
                    }
                    Err(_) => {
                        error!("System tray receiver disconnected.");
                        break;
                    }
                }
            }
        }
    });
}

use std::ffi::c_void;
extern "C" {
    fn CFRunLoopGetMain() -> *mut c_void;
    fn CFRunLoopWakeUp(rl: *mut c_void);
}

pub fn set_tray_title(new_title: &str) {
    TRAY_ICON.with(|tray| {
        if let Some(tray_icon) = tray.lock().unwrap().as_mut() {
            let _ = tray_icon.set_title(Some(new_title.to_string()));
        }
    });
}

pub fn init_tray() {
    // TODO: Implement tray menus
    // Currently, the main thread will panic, stating that MudaMenuItem class already exists or something,
    // probably because of tray_icon, dioxus, and dioxus-desktop using different version of muda.
    // Could use dioxus trayicon instead of tray_icon crate, and patch dioxus-desktop to use the same version of muda
    // as dioxus

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("AuraTimer: Time Remaining")
        .with_title("[25:00]")
        .build()
        .unwrap();

    // tray_icon cannot be dropped, otherwise the system tray will disappear
    TRAY_ICON.with(|t| {
        *t.lock().unwrap() = Some(tray_icon);
    });

    // redraw to make the system tray visible
    unsafe {
        let rl = CFRunLoopGetMain();
        if !rl.is_null() {
            CFRunLoopWakeUp(rl);
        }
    }
}
