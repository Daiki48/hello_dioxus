use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Entry(cx: Scope) -> Element {
    let item = use_state(&cx, || "".to_string());
    let article = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        style { [include_str!("../../style/entry.scss")] }
        h1 {
            "This page is Entry"
        }
        div {
            class: "itemarea",
            input {
                placeholder: "This is item",
                oninput: move |evt| {
                    item.set(evt.value.clone())
                }
            }
            br {}
            label {
                "{item}"
            }
        }
        textarea {
            rows: "10",
            cols: "60",
            placeholder: "This is article",
            oninput: move |e| {
                article.set(e.value.clone())
            }
        }
        p {
            "{article}"
        }

    ))
}
