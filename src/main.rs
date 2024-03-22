#![allow(non_snake_case)]

use std::collections::HashMap;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::layout::ThemeLayout;
use crate::model::{IngredientItem, sorted_ingredient_list};

mod layout;
mod model;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
#[component]
fn App(cx: Scope) -> Element {
    let ingredients = use_ref(cx, || Vec::<IngredientItem>::new());
    let adding = use_state(cx, || false);
    let name_to_add = use_state(cx, || String::new());

    render! {
        ThemeLayout{
            h1 { class: "text-4xl text-center p-8",
                "Label Creator"
            }
            div { class: "grid grid-flow-col gap-2",
                div { class: "flex flex-col",
                    h2 { class: "pb-4",
                        "Zutaten"
                        if ingredients.read().len() > 0 {
                            rsx! {
                                table { class: "table border-solid",
                                    tr {
                                        th {
                                            "Zutat"
                                        }
                                        th {
                                            "Menge"
                                        }
                                    }
                                    for ingredient in ingredients.read().clone() {
                                        tr {
                                            td {
                                                {ingredient.basicInfo.name}
                                            }
                                            td {
                                                input {
                                                        r#type: "number",
                                                        placeholder: "",
                                                        class: "input input-bordered input-accent",
                                                        oninput: move |evt| name_to_add.set(evt.value.clone())
                                                }
                                                "g"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            if *adding.get() == true {

                            // if true {
                                rsx! {
                                    div { class: "flex",
                                    input {
                                            r#type: "flex",
                                            placeholder: "Name",
                                            class: "input input-bordered input-accent",
                                            oninput: move |evt| name_to_add.set(evt.value.clone())
                                    }
                                    button { class: "btn btn-outline",
                                        onclick: move |evt|  {
                                            ingredients.write().push(
                                                IngredientItem::from_name(String::from(name_to_add.get()))
                                            );
                                            adding.set(false);
                                        },
                                        "Hinzufügen"
                                    }
                                    }
                                }
                            } else {
                                rsx! {
                                    button { class: "btn btn-outline",
                                        onclick: move |evt|  {
                                            adding.set(true);
                                        },
                                        "Eine Zutat hinzufügen"
                                    }
                                }
                            }
                        }
                    }
                },
                div { class: "flex flex-col",
                    h2 { class: "pb-4",
                        "Etiketten Vorschau"
                    }

                    if ingredients.read().len() > 0 {
                        rsx! {
                            h3 {
                                "Zutaten"
                            }
                            span {
                                sorted_ingredient_list(ingredients.read().clone())
                            }
                        }
                    }
                },
            }
        }
    }
}
