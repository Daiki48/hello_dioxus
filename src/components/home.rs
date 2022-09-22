use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("../../static/home.scss")] }
        h1 {
            "This page is Home"
        }
    ))
}
