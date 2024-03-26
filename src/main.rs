#![allow(non_snake_case)]

use std::collections::BTreeMap;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::layout::ThemeLayout;
use crate::model::{food_db, IngredientItem, sorted_ingredient_list};

mod layout;
mod model;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
#[component]
fn App(cx: Scope) -> Element {
    // let ingredients = use_ref(cx, || Vec::<IngredientItem>::new());
    let ingredients = use_ref(cx, || BTreeMap::<usize, IngredientItem>::new());
    let adding = use_state(cx, || false);
    let name_to_add = use_state(cx, || String::new());
    let mut last_id = use_state(cx, || 0_usize);

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
                                        // let key = ingredient.0.clone();
                                        {
                                            let key = ingredient.0;
                                            let ingr  = ingredient.1.clone();
                                            rsx! {
                                                tr { key: "{key}",
                                                    td {
                                                        "{key}"
                                                    }
                                                    td {
                                                        {ingr.clone().basicInfo.standard_ingredient.name}
                                                    }
                                                    td {
                                                        input {
                                                                r#type: "number",
                                                                placeholder: "",
                                                                class: "input input-bordered input-accent",
                                                                oninput: move |evt| {
                                                                    let mut new_amount_ingredient = ingredient.1.clone();
                                                                    if let Ok(new_amount) = evt.value.clone().parse::<i32>() {
                                                                        new_amount_ingredient.basicInfo.amount = new_amount;
                                                                        ingredients.write().insert(key, new_amount_ingredient).unwrap();
                                                                    }
                                                                }
                                                        }
                                                        " g"
                                                    }
                                                    td {
                                                        button {
                                                            class: "btn btn-square",
                                                            dangerous_inner_html: r###"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>"###,
                                                            onclick: move |_| {
                                                                ingredients.write().remove(&key);
                                                            }
                                                        }
                                                    }
                                                }
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
                                            list: "ingredients",
                                            r#type: "flex",
                                            placeholder: "Name",
                                            class: "input input-bordered input-accent",
                                            oninput: move |evt| name_to_add.set(evt.value.clone()),
                                            datalist {
                                                id: "ingredients",
                                                for item in food_db().clone() {
                                                    option { value: "{item.0}" }
                                                }
                                            }
                                    }
                                    button { class: "btn btn-outline",
                                        onclick: move |evt|  {
                                            ingredients.write().insert(
                                                last_id + 1,
                                                IngredientItem::from_name(String::from(name_to_add.get()))
                                            );
                                            last_id += 1;
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
                            div {
                                class: "border",
                                h3 {
                                    "Zutaten"
                                }
                                span {
                                    dangerous_inner_html: "{sorted_ingredient_list(ingredients.read().clone())}"
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
