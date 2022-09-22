use dioxus::prelude::*;
use dioxus_router::{
    Link,
    Route,
    Router
};

mod components;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    cx.render(rsx!(
        Router {
            ul {
                Link { to: "/", li {"Go home"} }
                Link { to: "/counter", li {"Counter"} }
                Link { to: "/entry", li {"Entry"} }
            }
            Route { to: "/", "Home" }
            Route { to: "/counter", components::counter::Counter{} }
            Route { to: "/entry", components::entry::Entry{} }
        }
    ))
}
