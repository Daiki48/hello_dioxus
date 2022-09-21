use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        h1 {
            style { [include_str!("../style.scss")] }
            "Counter : {count}"
        }
        button {
            onclick: move |_| count += 1,
            "+"
        }
        button {
            onclick: move |_| count -= 1,
            "-"
        }
        h2 {
            "This is h2 tag"
        }
        div {
            class: "divtag",
            "This is div tag"
        }
        button {
            class: "btnplus",
            onclick: move |_| count += 10,
            "+10"
        }
        button {
            class: "btnminus",
            onclick: move |_| count -= 10,
            "-10"
        }
    ))
}
