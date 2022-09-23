use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Bbs(cx: Scope) -> Element {
    let title = use_state(&cx, || "".to_string());
    let content = use_state(&cx, || "".to_string());
    cx.render(rsx!(
        style { [include_str!("../../static/bbs.scss")] }
        h1 {
            "This page is BBS"
        }
        div {
            class: "container",
            input {
                placeholder: "title",
                oninput: move |e| {
                    title.set(e.value.clone())
                }
            }
            br {}
            textarea {
                rows: "10",
                cols: "60",
                placeholder: "content",
                oninput: move |e| {
                    content.set(e.value.clone())
                }
            }
            input {
                r#type: "submit",
            }
        }
    ))
}
