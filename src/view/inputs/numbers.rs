use dioxus::prelude::*;

#[component]
pub fn NumberInput<'a>(
    cx: Scope<'a>,
    value: String,
    min: i64,
    max: i64,
    label: &'a str,
    class: &'a str,
) -> Element<'a> {
    render! {
        div{
            class: "col",
            label{"{label}"},
            input{
                r#type : "number",
                class : "form-control {class}",
                value : "{value}",
                min : "{min}",
                max : "{max}",
                placeholder : "{label}",
                disabled : "true",
            }
        }
    }
}

#[derive(Props)]
pub struct NumberChangeEvent<'a> {
    on_change: EventHandler<'a, FormEvent>,
    value: String,
    label: &'a str,
    class: &'a str,
    min: i32,
    max: i32,
}

#[component]
pub fn NumberInputMut<'a>(cx: Scope<'a, NumberChangeEvent<'a>>) -> Element<'a> {
    render!(
        div{
            class: "col",
            label{cx.props.label}
            input {
                r#type: "number",
                onchange: move |evt| cx.props.on_change.call(evt),
                class: "form-control {cx.props.class}",
                value: "{cx.props.value}",
                placeholder : "{cx.props.label}",
                min : "{cx.props.min}",
                max : "{cx.props.max}",
            }
        }
    )
}
