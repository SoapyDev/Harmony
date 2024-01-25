use dioxus::prelude::*;

#[component]
pub fn SwitchInput<'a>(cx: Scope<'a>, value: bool, label: &'a str) -> Element<'a> {
    render! {
        div{
           class: "checkbox-wrapper",
                "{label}",
            label{
                class: "switch",
                input{
                    name: "switch-box",
                    r#type : "checkbox",
                    class : "switch-box",
                    checked : "{value}",
                    disabled : "true",
                }
                div{
                    class: "slider round",
                }
            }

        }
    }
}

#[derive(Props)]
pub struct SwitchChangeEvent<'a> {
    on_change: EventHandler<'a, FormEvent>,
    value: bool,
    label: &'a str,
}

#[component]
pub fn SwitchInputMut<'a>(cx: Scope<'a, SwitchChangeEvent<'a>>) -> Element<'a> {
    render! {
        div{
           class: "checkbox-wrapper",
                cx.props.label
            label{
                class: "switch",
                input{
                    onchange: move |evt| cx.props.on_change.call(evt),
                    name: "switch-box",
                    r#type : "checkbox",
                    class : "switch-box",
                    checked : "{cx.props.value}",
                }
                div{
                    class: "slider round",
                }
            }

        }
    }
}
