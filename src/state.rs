use dioxus::signals::GlobalSignal;

pub static TIMER_EXPIRED: GlobalSignal<bool> = GlobalSignal::new(|| false);
pub static IS_FOCUS_MODE: GlobalSignal<bool> = GlobalSignal::new(|| true);

pub static SMALL_SESSION_COUNT: GlobalSignal<u32> = GlobalSignal::new(|| 0);
pub static FULL_SESSION_COUNT: GlobalSignal<u32> = GlobalSignal::new(|| 0);
