use dioxus::prelude::*;

#[warn(non_snake_case)]
pub fn Counter(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        h1 {
            style { [include_str!("../../static/style.scss")] }
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
