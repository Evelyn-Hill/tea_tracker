#![allow(non_snake_case)]

use std::io::Read;

use chrono::{DateTime, Local, Utc};
use dioxus::{desktop::tao::error, prelude::*};
use serde::{self, Deserialize, Serialize};
use tea_tracker::sqlite;
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
    milk: bool,
    sugar: bool,
    score: i8,
    description: String,
}

// TODO: Implement a "new" function.
impl TeaData {
    pub fn new(
        tea_type: String,
        score: i8,
        milk: bool,
        sugar: bool,
        description: String,
    ) -> TeaData {
        TeaData {
            date_time: chrono::Utc::now(),
            tea_type,
            milk,
            sugar,
            score,
            description,
        }
    }
}

const TEA_TYPE_PLACEHOLDER: &str = "Tea Type";
const DESCRIPTION_PLACEHOLDER: &str = "Description";

#[component]
fn App() -> Element {
    rsx! {
        tea_input_form {}
    }
}

//TODO: Make this pretty.
#[component]
fn tea_input_form() -> Element {
    let mut tea_type = use_signal(|| "".to_string());
    let mut sugar = use_signal(|| false);
    let mut milk = use_signal(|| false);
    let mut score = use_signal(|| 0);
    let mut description = use_signal(|| "".to_string());

    rsx! {
        div { max_width: "fit-content", max_height: "fit-content", margin_left: "auto", margin_right: "auto", margin_top: "auto", margin_bottom: "auto",
            form { onsubmit: move |_| {
                let td = TeaData::new(tea_type.read().to_string(), *score.read(), *milk.read(), *sugar.read(), description.read().to_string());
                // Tea data is being set, now to decide on how we will store it in database.
                println!("{}, {:?}, {:?}, {:?}, {}", td.tea_type, td.score, td.milk, td.sugar, td.description);
                },
                input { display: "flex", flex_direction: "row", width: "100%",
                        r#type: "text",
                        placeholder: "{TEA_TYPE_PLACEHOLDER}",
                        required: true,
                        oninput: move |event| tea_type.set(event.value()),
                }
                div { display: "flex", flex_direction: "column", align_items: "center", justify_content: "center",
                    div {
                        input { r#type: "checkbox", oninput: move |event| milk.set(event.checked())}
                        label { "Milk?" }
                    }
                    div {
                        input { r#type: "checkbox", oninput: move |event| sugar.set(event.checked())}
                        label { "Sugar?" }
                    }
                },
                div { display: "flex", flex_direction: "row", align_items: "center", justify_content: "center",
                    select {
                        oninput: move |event| score.set(event.value().parse().unwrap()),
                        option {"1"}
                        option {"2"}
                        option {"3"}
                        option {"4"}
                        option {"5"}
                    }
                }
                div { max_width: "fit_content",
                    textarea { min_height: "10rem", max_width: "fit-content", resize: "vertical",
                        placeholder: "{DESCRIPTION_PLACEHOLDER}", oninput: move |event| description.set(event.value()), }
                }
                div { display: "flex", flex_direction: "row", align_items: "center", justify_content: "center",
                    input { margin_left: "auto", margin_right: "auto", r#type: "submit"}
                }
            }
        }
    }
}
