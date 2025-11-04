// use crate::Route;
// use dioxus::prelude::*;

// #[component]
// pub fn Header() -> Element {
//     rsx! {
//         // Full header container
//         header {
//             class: "
//                 fixed top-0 left-0 w-full
//                 bg-[#BA0C2F]
//                 shadow-sm
//                 z-50
//             ",

//             // Inner flexbox layout
//             div {
//                 class: "
//                     max-w-7xl mx-auto
//                     flex items-center justify-between
//                     px-8 py-4
//                 ",

//                 // Left side — site title or logo
//                 h1 {
//                     class: "z-50 text-xl font-semibold text-white tracking-tight",
//                     "EcoClimate Variability Lab"
//                 }

//                 // Right side — navigation links
//                 nav {
//                     class: "z-50 flex gap-6 text-white font-medium",

//                     Link {
//                         to: Route::Home {},
//                         class: "hover:text-black transition-colors",
//                         "Home"
//                     }
//                     Link {
//                         to: Route::People {},
//                         class: "hover:text-black transition-colors",
//                         "People"
//                     }
//                     Link {
//                         to: Route::Facilities {},
//                         class: "hover:text-black transition-colors",
//                         "Facilities"
//                     }
//                     Link {
//                         to: Route::Publications {},
//                         class: "hover:text-black transition-colors",
//                         "Publications"
//                     }
//                 }
//             }
//         }

//         // The Outlet renders the current route's content below the header
//         main {
//             class: "pt-20", // adds spacing below the fixed header
//             Outlet::<Route> {}
//         }
//     }
// }
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Header() -> Element {
    let mut show_menu = use_signal(|| false);
    
    rsx! {
        header {
            class: "fixed top-0 left-0 w-full bg-[#BA0C2F] shadow-sm z-50",
            div {
                class: "max-w-7xl mx-auto flex items-center justify-between px-8 py-4",
                h1 { class: "z-50 text-xl font-semibold text-white tracking-tight", "EcoClimate Variability Lab" },
                // Desktop nav
                nav {
                    class: "hidden md:flex gap-6 text-white font-medium",
                    Link { to: Route::Home {}, class: "hover:text-black transition-colors", "Home" }
                    Link { to: Route::People {}, class: "hover:text-black transition-colors", "People" }
                    Link { to: Route::Facilities {}, class: "hover:text-black transition-colors", "Facilities" }
                    Link { to: Route::Publications {}, class: "hover:text-black transition-colors", "Publications" }
                },
                // Mobile hamburger
                button {
                    class: "md:hidden z-50 text-white text-2xl",
                    onclick: move |_| show_menu.set(!show_menu()),
                    "☰"
                }
            },
            // Mobile menu
            div {
                class: format!(
                    "md:hidden bg-[#BA0C2F] text-white flex flex-col px-8 py-4 gap-4 {}",
                    if show_menu() { "block" } else { "hidden" }
                ),
                Link { 
                    to: Route::Home {}, 
                    class: "hover:text-black transition-colors",
                    onclick: move |_| show_menu.set(false),
                    "Home" 
                }
                Link { 
                    to: Route::People {}, 
                    class: "hover:text-black transition-colors",
                    onclick: move |_| show_menu.set(false),
                    "People" 
                }
                Link { 
                    to: Route::Facilities {}, 
                    class: "hover:text-black transition-colors",
                    onclick: move |_| show_menu.set(false),
                    "Facilities" 
                }
                Link { 
                    to: Route::Publications {}, 
                    class: "hover:text-black transition-colors",
                    onclick: move |_| show_menu.set(false),
                    "Publications" 
                }
            }
        },
        main { class: "pt-20", Outlet::<Route> {} }
    }
}