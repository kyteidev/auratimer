use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    thread,
};

use dioxus::{desktop::window, prelude::*};
use tracing::error;

use crate::{
    components::timer::start_timer,
    state::{ALERT_WINDOW_ID, IS_FOCUS_MODE_MUTEX, SMALL_SESSION_COUNT_MUTEX},
    ui::button::Button,
};

#[derive(Debug)]
pub enum TimerCommand {
    Start,
}

pub static TIMER_EVENT_SENDER: Mutex<Option<Sender<TimerCommand>>> = Mutex::new(None);
pub static TIMER_EVENT_RECEIVER: Mutex<Option<Receiver<TimerCommand>>> = Mutex::new(None);
pub static TIMER_COMMAND_SENDER: Mutex<Option<Sender<TimerCommand>>> = Mutex::new(None);
pub static TIMER_COMMAND_RECEIVER: Mutex<Option<Receiver<TimerCommand>>> = Mutex::new(None);

// idk why but I need to route this mpsc channel to send to another mpsc channel so that main thread won't crash idk aaaaaa
// I have no idea why this works :/
// Probably gonna optimize this later
pub fn init_timer_event_listener() {
    thread::spawn(move || loop {
        if let Some(receiver) = TIMER_EVENT_RECEIVER.lock().unwrap().as_ref() {
            match receiver.recv() {
                Ok(TimerCommand::Start) => {
                    if let Some(sender) = TIMER_COMMAND_SENDER.lock().unwrap().as_ref() {
                        let _ = sender.send(TimerCommand::Start);
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

pub fn handle_timer_commands() {
    if let Some(receiver) = TIMER_COMMAND_RECEIVER.lock().unwrap().as_ref() {
        while let Ok(command) = receiver.try_recv() {
            match command {
                TimerCommand::Start => {
                    start_timer();
                }
            }
        }
    }
}

#[component]
pub fn Alert() -> Element {
    let is_focus_mode = *IS_FOCUS_MODE_MUTEX.lock().unwrap();
    let small_session_count = *SMALL_SESSION_COUNT_MUTEX.lock().unwrap();

    use_hook(|| {
        *ALERT_WINDOW_ID.lock().unwrap() = Some(window().id());
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {
            class: "w-screen h-screen bg-red-200 text-red-500 font-bold text-4xl rounded-lg p-4 flex justify-between items-center",
            h1 {
                if is_focus_mode {
                    "It's time to focus!"
                } else {
                    if small_session_count % 4 == 0 && small_session_count != 0 {
                        "Time for a long break!"
                    } else {
                        "Time for a short break!"
                    }
                }
            }
            Button {
                action: move || {
                    if let Some(sender) = TIMER_EVENT_SENDER.lock().unwrap().as_ref() {
                        let _ = sender.send(TimerCommand::Start);
                    }

                    window().close();
                },
                class: "bg-red-500 text-red-200 w-32 h-12 text-xl",
                title: if is_focus_mode {
                    "Start focus"
                } else {
                    "Start break"
                },
                text: if is_focus_mode {
                    "Start focus"
                } else {
                    "Start break"
                }
            }
        }
    }
}
