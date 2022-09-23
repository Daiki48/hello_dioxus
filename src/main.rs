use dioxus::prelude::*;
use dioxus_router::{
    Link,
    Route,
    Router
};

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;

mod components;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    cx.render(rsx!(
        style { [include_str!("../static/main.scss")] }
        Router {
            ul {
                li {
                    Link { to: "/", li {"Home"} }
                }
                li {
                    Link { to: "/counter", li {"Counter"} }
                }
                li {
                    Link { to: "/entry", li {"Entry"} }
                }
            }
            Route { to: "/", components::home::Home{} }
            Route { to: "/counter", components::counter::Counter{} }
            Route { to: "/entry", components::entry::Entry{} }
            Route { to: "", "Error 404 Not Found" }
        }
        footer {
            p {
                "©︎ 2022 Daiki",
                a {
                    class: "icongithub",
                    href: "https://github.com/Daiki48/hello_dioxus",
                    target: "_blank",
                    Icon {
                        width: 20,
                        height: 20,
                        icon: FaGithub,
                    }
                }
            }
        }
    ))
}
