use dioxus::prelude::*;

#[component]
pub fn Date<'a>(cx: Scope<'a>, value: String, placeholder: &'a str, class: &'a str) -> Element<'a> {
    render! {
        div{
            class: "col",
            label{
                "{placeholder}",
                },
            input{
                r#type : "date",
                class : "form-control {class}",
                value : "{value}",
                disabled : "true",
            }
        }
    }
}

#[derive(Props)]
pub struct DateChangeEvent<'a> {
    on_change: EventHandler<'a, FormEvent>,
    value: String,
    label: &'a str,
    class: &'a str,
}

#[component]
pub fn DateMut<'a>(cx: Scope<'a, DateChangeEvent>) -> Element<'a> {
    render!(
        div{
            class: "col",
            label{cx.props.label}
            input {
                r#type: "date",
                onchange: move |evt| cx.props.on_change.call(evt),
                class: "form-control {cx.props.class}",
                value: "{cx.props.value}",
            }
        }
    )
}
