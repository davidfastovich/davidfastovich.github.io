use csv::ReaderBuilder;
use wasm_bindgen_futures::spawn_local;
use dioxus::prelude::*;
use reqwest::Client;

#[derive(Debug, Clone)]
struct PubRow {
    year: String,
    title: String,
    authors: String,
    journal: String,
    website: String,
    note: String,
}

async fn fetch_publications() -> Option<Vec<PubRow>> {
    let url: &'static str = "https://docs.google.com/spreadsheets/d/1CcQRzGxZ4n-kqOkxQuq9pbpZtmaQ3rBLrul2tVf1ZSc/export?format=csv";
    let client: Client = Client::new();
    let resp = client.get(url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(resp.as_bytes());
    let mut rows = Vec::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            rows.push(PubRow {
                year: record.get(0).unwrap_or("").to_string(),
                title: record.get(1).unwrap_or("").to_string(),
                authors: record.get(2).unwrap_or("").to_string(),
                journal: record.get(3).unwrap_or("").to_string(),
                website: record.get(4).unwrap_or("").to_string(),
                note: record.get(5).unwrap_or("").to_string(),
            });
        }
    }
    Some(rows)
}

#[component]
fn PublicationEntry(
    year: String,
    title: String,
    authors: String,
    journal: String,
    website: String,
    note: String,
) -> Element {
    rsx! {
        a {
            href: "{website}",
            target: "_blank",
            class: "block mb-10",
            
            h3 {
                class: "text-lg font-semibold",
                "{title}"
            }
            
            p {
                class: "text-gray-700 mb-1",
                "{authors}"
            }
            
            div {
                class: "flex flex-wrap items-center gap-2 text-sm",
                span {
                    class: "font-medium text-red-700 italic",
                    "{journal}"
                }
                if !note.is_empty() {
                    span {
                        class: "text-xs italic",
                        "{note}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Publications() -> Element {
    let mut publications = use_signal(|| Vec::new());

    use_effect({
        move || {
            spawn_local(async move {
                if let Some(rows) = fetch_publications().await {
                    publications.set(rows);
                }
            });
        }
    });

    rsx! {
        div {
            div {class: "max-w-4xl mx-auto p-6 gap-8",
                    h1 {
                        class: "text-4xl font-bold mb-10",
                        "Publications"
                    }

                    hr { class: "border-gray-300 mb-10" }

                // Publications List
                div {
                    {
                        if publications().is_empty() {
                            rsx!(div { class: "text-black", "Loading ..." })
                        } else {
                            let mut years: Vec<String> = publications().iter().map(|p| p.year.clone()).collect();
                            years.sort_by(|a, b| b.cmp(a));
                            years.dedup();
                            let last_year = years.last().cloned().unwrap_or_default();
                            rsx! {{
                                years.into_iter().map(|year| {
                                    rsx! {
                                        div {  
                                            h2 { class: "text-2xl font-bold mb-10", "{year}" }

                                            {
                                                publications().iter().filter(|p| p.year == year).map(|p| {
                                                    rsx! {
                                                        PublicationEntry {
                                                            year: p.year.clone(),
                                                            title: p.title.clone(),
                                                            authors: p.authors.clone(),
                                                            journal: p.journal.clone(),
                                                            website: p.website.clone(),
                                                            note: p.note.clone(),
                                                        }
                                                    }
                                                })
                                            }
                                        }

                                        if &year != &last_year {
                                            hr { class: "border-gray-300 mb-10" }
                                        }
                                    }
                                })
                            }}
                        }
                    }
                }
            }
        }
    }
}
