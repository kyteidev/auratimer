use dioxus::{
    hooks::use_effect,
    signals::{GlobalSignal, Readable},
};

pub static TIMER_EXPIRED: GlobalSignal<bool> = GlobalSignal::new(|| false);
pub static IS_FOCUS_MODE: GlobalSignal<bool> = GlobalSignal::new(|| true);

pub static SMALL_SESSION_COUNT: GlobalSignal<u32> = GlobalSignal::new(|| 0);
pub static FULL_SESSION_COUNT: GlobalSignal<u32> = GlobalSignal::new(|| 0);

pub static BG_COLOR: GlobalSignal<&str> = GlobalSignal::new(|| "bg-blue-200");
pub static BG_COLOR_HOVER: GlobalSignal<&str> = GlobalSignal::new(|| "hover:bg-blue-500");
pub static BG_COLOR_INVERTED: GlobalSignal<&str> = GlobalSignal::new(|| "bg-blue-500");
pub static TEXT_COLOR: GlobalSignal<&str> = GlobalSignal::new(|| "text-blue-500");
pub static TEXT_COLOR_INVERTED: GlobalSignal<&str> = GlobalSignal::new(|| "text-blue-200");
pub static ICON_COLOR: GlobalSignal<&str> = GlobalSignal::new(|| "fill-blue-500 stroke-blue-500");

pub fn init_colors() {
    use_effect(|| {
        let timer_expired = *TIMER_EXPIRED.read();
        let is_focus_mode = *IS_FOCUS_MODE.read();

        let bg_color = if timer_expired {
            "bg-red-200"
        } else if is_focus_mode {
            "bg-blue-200"
        } else {
            "bg-green-200"
        };

        let bg_hover = if timer_expired {
            "hover:bg-red-500"
        } else if is_focus_mode {
            "hover:bg-blue-500"
        } else {
            "hover:bg-green-500"
        };

        let bg_color_inverted = if timer_expired {
            "bg-red-500"
        } else if is_focus_mode {
            "bg-blue-500"
        } else {
            "bg-green-500"
        };

        let text_color = if timer_expired {
            "text-red-500"
        } else if is_focus_mode {
            "text-blue-500"
        } else {
            "text-green-500"
        };

        let text_color_inverted = if timer_expired {
            "text-red-200"
        } else if is_focus_mode {
            "text-blue-200"
        } else {
            "text-green-200"
        };

        let icon_color = if timer_expired {
            "fill-red-500 stroke-red-500"
        } else if is_focus_mode {
            "fill-blue-500 stroke-blue-500"
        } else {
            "fill-green-500 stroke-green-500"
        };

        *BG_COLOR.write() = bg_color;
        *BG_COLOR_HOVER.write() = bg_hover;
        *BG_COLOR_INVERTED.write() = bg_color_inverted;
        *TEXT_COLOR.write() = text_color;
        *TEXT_COLOR_INVERTED.write() = text_color_inverted;
        *ICON_COLOR.write() = icon_color;
    });
}
