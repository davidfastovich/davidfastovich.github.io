// use crate::components::Hero;
use dioxus::prelude::*;

#[component]
pub fn People() -> Element {
    rsx!(
        div { class: "max-w-4xl mx-auto p-6 gap-8",
            h1 { class: "text-4xl font-bold mb-10", "People" }

            hr { class: "border-gray-300 mb-10" }
        
            div { class: "",
                h2 {class: "text-2xl font-bold mb-10", "Principal Investigator"}
                }

            div { class: "flex flex-col md:flex-row items-center gap-6 mb-10",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/fastovich.jpeg"),
                        alt: "David Fastovich"
                    }

                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "David Fastovich"}
                    h3 {class: "text-l italic mb-4", "Assistant Professor, Geography, University of Georgia"}
                    p {
                        "My research focuses on understanding temperature and rainfall changes throughout
                    Earth's recent and distant past, from the tropical Pacific to the U.S. Atlantic coast.
                    I'm particularly interested in how these climate shifts forced forest ecosystems to
                    reshuffle and adapt to new environmental pressures."
                      }
                    a {
                        href:  "https://drive.google.com/uc?export=download&id=1iFK2Qfe2tseO369TxHCrg4MSJNQe6fqH",  // Replace with your PDF URL
                        target: "_blank",  // Opens in new tab
                        rel: "noopener noreferrer",  // Security best practice
                        class: "inline-block mt-4 mr-4",
                        button {
                            class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 hover:text-black transition-colors",
                            "CV"
                        }
                    }
                    a {
                        href:  "mailto:fastovich@uga.edu",  // Replace with your PDF URL
                        target: "_blank",  // Opens in new tab
                        rel: "noopener noreferrer",  // Security best practice
                        class: "inline-block mt-4 mr-4",
                        button {
                            class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 hover:text-black transition-colors",
                            "E-Mail"
                        }
                    }
                    a {
                        href:  "https://geography.uga.edu/directory/people/david-fastovich",  // Replace with your PDF URL
                        target: "_blank",  // Opens in new tab
                        rel: "noopener noreferrer",  // Security best practice
                        class: "inline-block mt-4 mr-4",
                        button {
                            class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 hover:text-black transition-colors",
                            "Website"
                        }
                    }

                    a {
                        href:  "https://scholar.google.com/citations?user=HhNaFTEAAAAJ&hl=en&oi=ao",  // Replace with your PDF URL
                        target: "_blank",  // Opens in new tab
                        rel: "noopener noreferrer",  // Security best practice
                        class: "inline-block mt-4",
                        button {
                            class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 hover:text-black transition-colors",
                            "Google Scholar"
                        }
                    }
                }

            
            }

            hr { class: "border-gray-300 mb-10" }

            div { class: "",
                h2 {class: "text-2xl font-bold mb-10", "Graduate Students"}
                }
            
            p {
                "If you're interested pursuing graduate education in ecology, climatology, or environmental data science,
                reach out to David Fastovich! Department providing teaching assistant opportunities and grant funded
                positions are available. You can find information on how to apply to the Geography department "

                a {
                    href:  "https://geography.uga.edu/how-apply-0",  // Replace with your PDF URL
                    target: "_blank",  // Opens in new tab
                    rel: "noopener noreferrer",  // Security best practice
                    class: "inline-block",
                    button {
                        class: "text-black underline hover:text-gray-500 visited:text-gray-500 transition-color",
                        "here"
                    }

                "."
                }
            }
        }
    )
}
