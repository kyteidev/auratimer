use std::{
    sync::{
        mpsc::{Receiver, Sender, TryRecvError},
        Mutex,
    },
    time::Duration,
};

use dioxus::prelude::spawn;
use once_cell::sync::Lazy;
use tokio::time::interval;
use tracing::error;
use tray_icon::{
    menu::{Menu, MenuEvent},
    TrayIconBuilder,
};

pub static TRAY_EVENT_SENDER: Lazy<Mutex<Option<Sender<MenuEvent>>>> =
    Lazy::new(|| Mutex::new(None));
pub static TRAY_EVENT_RECEIVER: Lazy<Mutex<Option<Receiver<MenuEvent>>>> =
    Lazy::new(|| Mutex::new(None));

thread_local! {
    static TRAY_ICON: Mutex<Option<tray_icon::TrayIcon>> = Mutex::new(None);
}

pub fn init_tray_handler() {
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        if let Some(sender) = TRAY_EVENT_SENDER.lock().unwrap().as_ref() {
            let _ = sender.send(event);
        }
    }));
}

pub fn init_tray_listener() {
    spawn(async move {
        let mut interval = interval(Duration::from_millis(200));
        loop {
            interval.tick().await;
            if let Some(receiver) = TRAY_EVENT_RECEIVER.lock().unwrap().as_ref() {
                match receiver.try_recv() {
                    Ok(menu_event) => {
                        println!("RECEIVED: {}", menu_event.id().0);
                        match menu_event.id().0.as_str() {
                            _ => {}
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
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
    let tray_menu = Menu::new();
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
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
