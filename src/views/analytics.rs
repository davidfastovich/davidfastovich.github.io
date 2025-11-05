use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Document, HtmlScriptElement, Window};

pub fn Analytics() -> Element {
    use_effect(|| {
        // Get window and document handles
        let window: Window = web_sys::window().unwrap();
        let document: Document = window.document().unwrap();

        // --- Load Google Tag (gtag.js) ---
        let gtm_script = document
            .create_element("script")
            .unwrap()
            .dyn_into::<HtmlScriptElement>()
            .unwrap();

        gtm_script.set_async(true);
        gtm_script.set_src("https://www.googletagmanager.com/gtag/js?id=G-JN7B9YD2EF");

        document
            .body()
            .unwrap()
            .append_child(&gtm_script)
            .unwrap();

        // --- Inline GA initialization script ---
        let inline_script = document
            .create_element("script")
            .unwrap()
            .dyn_into::<HtmlScriptElement>()
            .unwrap();

        inline_script.set_text(
            "
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
            gtag('config', 'G-JN7B9YD2EF');
            ",
        );

        document
            .body()
            .unwrap()
            .append_child(&inline_script)
            .unwrap();
    });

    // Return an empty Dioxus element instead of None
    rsx! {}
}