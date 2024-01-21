#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(App).launch();
}

fn App(cx: Scope) -> Element {
    let mut leaf = use_state(cx, || {
        brainace_core::Leaf::new("The first five digits of Pi are [...]", "3.1415")
    });

    render! {
        Leaf {
            leaf: leaf.get()
        }
    }
}

#[component]
fn Leaf<'a>(cx: Scope, leaf: &'a brainace_core::Leaf) -> Element {
    let front = leaf.quiz();
    let back = leaf.answer();

    render! {
        p { front }
        p { back }
    }
}
