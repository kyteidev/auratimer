use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    thread,
};

use dioxus::desktop::window;
use tracing::error;
use tray_icon::{TrayIconBuilder, TrayIconEvent};

#[derive(Debug)]
pub enum WindowCommand {
    Show,
}

pub static TRAY_EVENT_SENDER: Mutex<Option<Sender<TrayIconEvent>>> = Mutex::new(None);
pub static TRAY_EVENT_RECEIVER: Mutex<Option<Receiver<TrayIconEvent>>> = Mutex::new(None);
pub static WINDOW_COMMAND_SENDER: Mutex<Option<Sender<WindowCommand>>> = Mutex::new(None);
pub static WINDOW_COMMAND_RECEIVER: Mutex<Option<Receiver<WindowCommand>>> = Mutex::new(None);

thread_local! {
    static TRAY_ICON: Mutex<Option<tray_icon::TrayIcon>> = const { Mutex::new(None) };
}

pub fn init_tray_handler() {
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        if let Some(sender) = TRAY_EVENT_SENDER.lock().unwrap().as_ref() {
            let _ = sender.send(event);
        }
    }));
}

pub fn init_tray_listener() {
    thread::spawn(move || loop {
        if let Some(receiver) = TRAY_EVENT_RECEIVER.lock().unwrap().as_ref() {
            match receiver.recv() {
                Ok(tray_event) => {
                    if let TrayIconEvent::Click {
                        button: tray_icon::MouseButton::Left,
                        button_state: tray_icon::MouseButtonState::Up,
                        ..
                    } = tray_event
                    {
                        if let Some(sender) = WINDOW_COMMAND_SENDER.lock().unwrap().as_ref() {
                            let _ = sender.send(WindowCommand::Show);
                        }
                    }
                }
                Err(_) => {
                    error!("System tray receiver disconnected.");
                    break;
                }
            }
        }
    });
}

pub fn handle_window_commands() {
    if let Some(receiver) = WINDOW_COMMAND_RECEIVER.lock().unwrap().as_ref() {
        while let Ok(command) = receiver.try_recv() {
            let handle = window();
            match command {
                WindowCommand::Show => {
                    handle.set_visible(true);
                }
            }
        }
    }
}

use std::ffi::c_void;
extern "C" {
    fn CFRunLoopGetMain() -> *mut c_void;
    fn CFRunLoopWakeUp(rl: *mut c_void);
}

pub fn set_tray_title(new_title: &str) {
    TRAY_ICON.with(|tray| {
        if let Some(tray_icon) = tray.lock().unwrap().as_mut() {
            tray_icon.set_title(Some(new_title.to_string()));
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
        .with_title("Focus: 25:00")
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
