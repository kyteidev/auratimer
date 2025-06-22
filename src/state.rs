use dioxus::signals::GlobalSignal;

pub static TIMER_EXPIRED: GlobalSignal<bool> = GlobalSignal::new(|| false);
pub static IS_FOCUS_MODE: GlobalSignal<bool> = GlobalSignal::new(|| true);
