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
    let output = use_state(cx, || {
        "An astronaut riding a unicorn realistic.".to_string()
    });
    let prompt_result = use_future(cx, (output,), |(output,)| {
        img_gen::get_image(output.to_string())
    });
    cx.render(rsx! {
        div { class: "container mx-auto h-full w-full",
            p { class: "text-3xl text-center", "{output}" }
            match prompt_result.value(){
                Some(res) => {
                    rsx! {img { class: "aspect-square h-1/2 rounded-lg",src: res.as_str()}}
                },
                None => rsx! { p { class: "text-2xl text-center","Image loading..."} }
            },
            input {
                class: "border border-gray-300 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 mt-2",
                value: "{prompt}",
                placeholder: "Describe the image that you want.",
                oninput: move |evt| prompt.set(evt.value.clone())
            }
            button {
                class: "bg-violet-900 border border-gray-300 text-xl rounded-lg text-white focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 mt-2",
                onclick: move |_evt| {
                    output.set(prompt.to_string())
                },
                "Generate."
            }
        }
    })
}
