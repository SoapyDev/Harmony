use dioxus::prelude::*;

#[component]
pub fn TextArea<'a>(cx: Scope<'a>, value: String, label: &'a str, class: &'a str) -> Element<'a> {
    render! {
        div{
            class: "col",
            label{ "{label}" },
            textarea{
                class : "form-control {class}",
                placeholder : "{label}",
                value : "{value}",
                disabled : "true",
            }
        }
    }
}

#[derive(Props)]
pub struct TextAreaChangeEvent<'a> {
    on_change: EventHandler<'a, FormEvent>,
    on_input: EventHandler<'a, FormEvent>,
    value: String,
    label: &'a str,
    class: &'a str,
}

#[component]
pub fn TextAreaMut<'a>(cx: Scope<'a, TextAreaChangeEvent<'a>>) -> Element<'a> {
    render!(
        div{
            class: "col",
            label{cx.props.label}
            textarea {
                oninput: move |evt| cx.props.on_input.call(evt),
                onchange: move |evt| cx.props.on_change.call(evt),
                class: "form-control {cx.props.class}",
                value: "{cx.props.value}",
            }
        }
    )
}
