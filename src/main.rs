#![allow(non_snake_case)]

use chrono::{DateTime, Local, Utc};
use dioxus::{desktop::tao::error, prelude::*};
use serde::{self, Deserialize, Serialize};
use tracing::Level;

fn main() {
    // Init logger
    dioxus::launch(App);
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
}

#[derive(Serialize, Deserialize)]
struct TeaData {
    #[serde(with = "chrono::serde::ts_seconds")]
    date_time: chrono::DateTime<Utc>,
    tea_type: String,
    score: i8,
    milk: bool,
    sugar: bool,
}

// TODO: Implement a "new" function.
impl TeaData {}

const TEA_TYPE_PLACEHOLDER: &str = "Tea Type";

#[component]
fn App() -> Element {
    rsx! {
        tea_input_form {}
    }
}

//TODO: Make this pretty.
#[component]
fn tea_input_form() -> Element {
    let mut tea_type = use_signal(|| "Tea Type".to_string());
    let mut sugar = use_signal(|| false);
    let mut milk = use_signal(|| false);
    let mut score = use_signal(|| 1);

    rsx! {
        div {
            form { onsubmit: move |event| { println!("{tea_type}, {milk}, {sugar}, {score}") },
                input { r#type: "text", placeholder: "{TEA_TYPE_PLACEHOLDER}", required: true, oninput: move |event| tea_type.set(event.value())},
                    div {
                        ul {
                        input { r#type: "checkbox", oninput: move |event| milk.set(event.checked())}
                        "Milk?"
                    }
                    div {
                        input { r#type: "checkbox", oninput: move |event| sugar.set(event.checked())}
                        "Sugar?"
                    }
                },
                select { oninput: move |event| score.set(event.value().parse().unwrap()),
                    option {"1"}
                    option {"2"}
                    option {"3"}
                    option {"4"}
                    option {"5"}
                }
                input {r#type: "submit"}
            }
        }
    }
}
