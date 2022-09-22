use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let item = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        components::counter::Counter{}
        h2 {
            "This is h2 tag"
        }
        div {
            class: "divtag",
            "This is div tag"
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
