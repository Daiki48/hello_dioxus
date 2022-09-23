use dioxus::prelude::*;


#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("../../style/home.scss")] }
        h1 {
            "This page is Home"
        }
        p {
            "このサイトは、私のdioxus勉強用サイトです。使える機能を試すためのものです。ソースコードもgithubから確認出来ます。"
        }
        p {
            "This is my dioxus study site. It is for testing the features available. Source code is also available from github."
        }
    ))
}
