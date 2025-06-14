use dioxus::prelude::*;

#[component]
pub fn RoundButton() -> Element {
    rsx! {
        button {
            class: "rounded-full bg-blue-500 w-24 h-24",
        }
    }
}
