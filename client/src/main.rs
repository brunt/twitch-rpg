use leptos::html::Canvas;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    Effect, ElementChild, Get, NodeRef, NodeRefAttribute, request_animation_frame,
};
use leptos::{IntoView, component, view};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>{"Hello, World!"}</h1>
        <GameCanvas />
    }
}

#[component]
fn GameCanvas() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    // redraws the canvas every frame, needed for scrolling background
    request_animation_frame(move || {});

    // TODO: offscreen canvas 🤷🏻‍♂️
    Effect::new(move |_| {});

    view! { <canvas node_ref=canvas_ref width=1600 height=900 /> }
}
