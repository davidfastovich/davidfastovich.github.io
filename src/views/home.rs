// use crate::components::Hero;
use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::MdAnnouncement;
use dioxus_free_icons::Icon;

pub fn Home() -> Element {
    rsx!(
        div { class: "flex flex-col-reverse md:flex-row max-w-4xl mx-auto p-6 gap-8",
            // div { class: "md:w-1/2",
            //     h1 { class: "text-4xl font-bold mb-4", "Our Lab" }
            //     p { class: "text-lg leading-relaxed mb-6",
            //         "Earth's climate is changing faster than ever before. To understand what lies ahead, 
            //         we look to the past which offers a history of natural experiments many timescales. 
            //         From ice ages to the hothouse climates that allowed dinosaurs to roam the Earth, these
            //         ancient climates reveal how our planet responds to change. Our research group studies
            //         these dramatic  shifts throughout Earth's history and how forests and ecosystems have
            //         adapted to them."
            //     }
            //     p { class: "text-lg leading-relaxed mb-6",
            //         "We explore these questions through three complementary approaches:"
            //         ul { class: "list-disc list-inside mt-3 ml-6",
            //             li { "Analyzing ancient molecules preserved in sediments to reconstruct past climates." }
            //             li { "Using climate models to understand the forces that drove past climate changes." }
            //             li { "Synthesizing global records to reveal how forests adapted to changing temperatures and rainfall." }
            //         }
            //     }
            //     div { class: "flex border-2 border-yellow-500 rounded-xl p-4 bg-yellow-50 mt-6",
            //         div {
            //             class: "mr-4 self-center",
            //             Icon {
            //                 width: 30,
            //                 height: 30,
            //                 fill: "rgb(230, 179, 19)",
            //                 icon: MdAnnouncement,
            //             }
            //         }

            //         p { class: "text-yellow-500 text-sm font-semibold",
            //             "Lab opportunities available! Interested in any of these research themes? Reach out to " 
            //             a {
            //                 href:  "mailto:fastovich@uga.edu",  // Replace with your PDF URL
            //                 target: "_blank",  // Opens in new tab
            //                 rel: "noopener noreferrer",  // Security best practice
            //                 class: "inline-block",
            //                 button {
            //                     class: "text-yellow-500 underline hover:text-gray-500 visited:text-gray-500 transition-color",
            //                     "David Fastovich"
            //                 }
            //             " to join us!" 
            //         }
            //     }
            //     }

            // }
            // div { class: "flex justify-center items-center md:w-1/2 mb-6 md:mb-0",
            //     img{
            //         class: "rounded-2xl h-128 w-auto object-cover self-center",
            //         src: asset!("assets/img/home.jpeg")
            //     }
            // }

            div { class: "max-w-4xl mx-auto p-6",
                h1 { class: "text-4xl font-bold mb-4", "Our Lab" }

                div {
                    class: "relative",
                    img {
                        class: "float-right ml-6 mb-4 rounded-2xl w-110 h-auto object-cover",
                        src: asset!("assets/img/home.jpeg"),
                        alt: "Lake Tulane, Florida"
                    }

                    p { class: "text-lg leading-relaxed mb-6",
                        "Earth's climate is changing faster than ever before. To understand what lies ahead,
                        our lab looks to the past which offers a history of natural experiments many timescales.
                        From ice ages that allowed mammoths to thrive to the hothouse climates that allowed dinosaurs
                        to roam the Earth, these ancient climates reveal how our planet responds to change. Our research
                        group studies these dramatic shifts in climate throughout Earth's history and how forests and
                        ecosystems have adapted to them. We are particularly interested in identifying the dynamics
                        of past climate changes which involves combing empirical evidence of past climate changes
                        alongside physics-based models. Based in the Department of Geography at the University of Georgia,
                        we conduct this research using state-of-the-art analytical facilities at the Center for Applied
                        Isotope Studies."
                    }

                    p { class: "text-lg leading-relaxed mb-6",
                        "We explore these questions through three complementary approaches:"
                    }

                    ul { class: "text-lg list-disc list-inside mt-3 ml-6",
                        li { "Analyzing ancient molecules preserved in sediments to reconstruct past climates." }
                        li { "Using climate models to understand the forces that drove past climate changes." }
                        li { "Synthesizing global records to reveal how forests adapted to changing temperatures and rainfall." }
                    }

                    div { class: "flex border-2 border-yellow-500 rounded-xl p-4 bg-yellow-50 mt-6 clear-both",
                        div {
                            class: "mr-4 self-center",
                            Icon {
                                width: 30,
                                height: 30,
                                fill: "rgb(230, 179, 19)",
                                icon: MdAnnouncement,
                            }
                        }

                        p { class: "text-yellow-500 text-sm font-semibold",
                            "Lab opportunities available! Interested in any of these research themes? Reach out to " 
                            a {
                                href: "mailto:fastovich@uga.edu",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "inline-block",
                                button {
                                    class: "text-yellow-500 underline hover:text-gray-500 visited:text-gray-500 transition-color",
                                    "David Fastovich"
                                }
                            }
                            " to join us!"
                        }
                    }
                }
            }

        }
    )
}