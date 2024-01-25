use dioxus::prelude::*;

#[component]
pub fn TextInput<'a>(cx: Scope<'a>, value: String, label: &'a str, class: &'a str) -> Element<'a> {
    render! {
        div{
            class: "col",
            label{"{label}"},
            input{
                r#type : "text",
                placeholder : "{label}",
                class : "form-control {class}",
                value : "{value}",
                disabled : "true",
            }
        }
    }
}

#[derive(Props)]
pub struct TextChangeEvent<'a> {
    on_input: EventHandler<'a, FormEvent>,
    on_change: EventHandler<'a, FormEvent>,
    value: String,
    label: &'a str,
    class: &'a str,
}

#[component]
pub fn TextInputMut<'a>(cx: Scope<'a, TextChangeEvent<'a>>) -> Element<'a> {
    render!(
        div{
            class: "col",
            label{cx.props.label}
            input {
                r#type: "text",
                oninput: move |evt| cx.props.on_input.call(evt),
                onchange: move |evt| cx.props.on_change.call(evt),
                class: "form-control {cx.props.class}",
                value: "{cx.props.value}",
            }
        }
    )
}
