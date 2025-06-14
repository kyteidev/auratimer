use dioxus::prelude::*;

#[component]
pub fn Timer() -> Element {
    let mut hovering = use_signal(|| false);
    rsx! {
        div {
            class: "bg-transparent hover:bg-blue-100 hover:opacity-50 w-3/5 h-2/5 rounded-lg text-[10rem] content-center text-center",
            onmouseenter: move |_| hovering.set(true),
            onmouseleave: move |_| hovering.set(false),
            "25:00"
        }
    }
}
