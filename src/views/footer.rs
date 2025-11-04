use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div{ class: "max-w-4xl mx-auto p-6 gap-8",
            
            hr { class: "border-gray-300 mb-10" }
            
        }
    }
}
