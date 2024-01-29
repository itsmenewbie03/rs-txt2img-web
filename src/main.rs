#![allow(non_snake_case)]
mod img_gen;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let prompt = use_state(cx, || "".to_string());
    let output = use_state(cx, || "Sample Image Below".to_string());
    let prompt_result = use_future(cx, (output,), |(output,)| img_gen::get_image(output.to_string()));
    cx.render(rsx! {
        div {
            p {
                "{output}"
            }
            match prompt_result.value(){
                Some(res) => {
                    rsx! {img { src: res.as_str()}}
                },
                None => rsx! { p { "Failed to Fetch Image!"}}
            }
            img {
                src: "https://raw.githubusercontent.com/itsmenewbie03/itsmenewbie03/main/shit_1-modified.png",
                style: "width: 30%; heigth: 30%;"
            }
            input {
                value: "{prompt}",
                oninput: move |evt| prompt.set(evt.value.clone())
            }
            button {
                onclick: move |_evt| output.set(prompt.to_string()),
                "Generate."
            }
        }
    })
}
