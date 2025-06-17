use dioxus::signals::GlobalSignal;

pub static TIMER_EXPIRED: GlobalSignal<bool> = GlobalSignal::new(|| false);
