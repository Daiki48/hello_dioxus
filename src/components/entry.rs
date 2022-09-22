use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Entry(cx: Scope) -> Element {
    let item = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        h1 {
            style { [include_str!("../../static/entry.scss")] }
            "This page is Entry"
        }
        input {
            oninput: move |evt| {
                item.set(evt.value.clone())
            }
        }
        label {
            "item is {item}"
        }

    ))
}
