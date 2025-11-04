// use crate::components::Hero;
use dioxus::{prelude::*};

#[component]
pub fn Facilities() -> Element {
    rsx!(
        div { class: "max-w-4xl mx-auto p-6 gap-8",
            h1 { class: "text-4xl font-bold mb-10", "Facilites" }

            hr { class: "border-gray-300 mb-10" }

            div { class: "mb-10", 
                p { class: "mb-4",
                    "The Earth & Climate Variability Lab is a fully equipped organic geochemistry facility housed in the University
                    of Georgia's Center for Applied Isotope Studies. Our comprehensive facilities include all instruments, materials,
                    and equipment needed to extract, purify, quantify, and measure a variety of organic biomarkers. This includes
                    ample fume hood space that is spread out across the building."
                }

                p { class: "mb-4",
                    "We specialize in compound-specific isotope analysis, currently offering hydrogen and carbon isotope measurements
                    with the capability to extend our services to nitrogen and oxygen isotopes. While our primary focus is on leaf
                    waxes, we also have extensive experience with glycerol dialkyl glycerol tetraethers (GDGTs) and can quantify sea
                    surface temperatures using the alkenone paleothermometer."
                }

                p { class: "mb-4",
                    "We are always eager to collaborate, so please feel free to reach out with any questions about our capabilities or
                    potential projects!"
                }

                p { class: "mb-4",
                    "Outside of our analytical facilities, our climate model analyses and data syntheses are supported by the "
                    
                    a { class: "text-black underline hover:text-gray-500 visited:text-gray-500 transition-colors",
                    href: "https://gacrc.uga.edu",
                    target: "_blank", "Georgia Advanced Computing Resource Center's" 
                      }
                    
                    " Sapelo2 computing cluster. While this high-performance computing cluster offers various configurations, we
                    primarily utilize the high memory nodes."
                }
            }

            div { class: "flex flex-col md:flex-row items-center gap-6 mb-10",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/coke.jpeg"),
                        alt: "GC-IRMS"
                    }
                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "Trace 1310 - GC Isolink II - Conflo IV - Delta V Advantage"}
                    h3 {class: "text-l italic mb-4", "Coke"}
                    p {"This instrument enables compound-specific isotopic measurements and is currently configured to analyze
                    hydrogen and carbon isotopes in leaf waxes (n-alkanes and n-alkanoic acids) with 20-30 carbon chains.
                    The system includes a TriPlus RSH autosampler and can also perform nitrogen and oxygen isotopic measurements
                    when needed. While not currently connected, it can be paired with an ISQ-QD mass spectrometer to simultaneously
                    identify compound structures and determine their isotopic composition."}
                    }
                }

            div { class: "flex flex-col md:flex-row items-center gap-6 mb-10",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/chuck.jpeg"),
                        alt: "GC-FID"
                    }
                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "Agilent 7890A GC-FID"}
                    h3 {class: "text-l italic mb-4", "Chuck"}
                    p {"This instrument quantifies target compound abundances before isotopic measurement, making it the workhorse
                    of our lab. Regardless of which compounds we're analyzing, we always identify and quantify them on this
                    instrument first."}
                    }
                }

            div { class: "flex flex-col md:flex-row items-center gap-6 mb-10",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/kyle.jpeg"),
                        alt: "Dionex ASE 200"
                    }
                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "Dionex ASE 200"}
                    h3 {class: "text-l italic mb-4", "Kyle and ASE2 (not pictured)"}
                    p {"This instrument is our primary method for extracting organic lipids from sediments. It operates like an
                    espresso maker, using pressure and solvent to extract organic compounds from sediment samples. While Kyle and
                    ASE2 may be as old as the lab PI, both instruments still work wonderfully."}
                    }
                }

            div { class: "flex flex-col md:flex-row items-center gap-6 mb-10",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/orange.jpeg"),
                        alt: "Horizon TurboVap"
                    }
                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "Horizon XcelVap"}
                    h3 {class: "text-l italic mb-4", "Orange"}
                    p {"This instrument quickly and cleanly dries down solvents that come off the ASE 200's. It has a smaller sibling
                    (FlexiVap Workstation) that is used for drying smaller volumes of solvents."}
                    }
                }

            div { class: "flex flex-col md:flex-row items-center gap-6",

                div { class: "w-full md:w-1/4 flex-shrink-0",
                    img { class: "rounded-2xl w-full h-auto",
                        src: asset!("assets/img/furnace.jpeg"),
                        alt: "Muffle Furnace"
                    }
                }

                div { class: "w-full md:w-3/4 scrollbar-hide",
                    h2 {class: "text-2xl font-bold mb-4", "Fisher Scientific Isotemp Muffle Furnace"}
                    h3 {class: "text-l italic mb-4", "Ash"}
                    p {"We routinely use this muffle furnace to bake our glassware at 450°C to clean and remove any possible
                    organic contaminants."}
                    }
                }

        }
    )
}
