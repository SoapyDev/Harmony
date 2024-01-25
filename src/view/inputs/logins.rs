use dioxus::prelude::*;
use crate::model::users::user_login::UserLogin;


#[component]
pub fn Username<'a>(
    cx: &'a Scoped<'a>,
    min: i64,
    max: i64,
    label: &'a str,
    user: UseRef<UserLogin>,
) -> Element<'a> {
    render! {
        div{
            label{
                font_family: "inter",
                font_size: 16,
                font_weight: 400,
                "{label}"
            },
            input{
                r#type: "text",
                minlength: *min,
                maxlength: *max,
                oninput: move |evt| {
                    user.with_mut(|val| val.set_username(evt.value.clone()));
                    user.with_mut(|val| val.is_valid());
                }
            }
        }
    }
}

#[component]
pub fn Password<'a>(
    cx: &'a Scope<'a>,
    min: i64,
    max: i64,
    label: &'a str,
    user: UseRef<UserLogin>,
) -> Element<'a> {
    render! {
        div{
            label{
                font_family: "inter",
                font_size: 16,
                font_weight: 400,
                "{label}"
            },
            input{
                r#type: "password",
                font_family: "inter",
                font_size: 16,
                font_weight: 400,
                minlength: *min,
                maxlength: *max,
                oninput:move |evt| {
                    user.with_mut(|val| val.set_password(evt.value.clone()));
                    user.with_mut(|val| val.is_valid());
                }
            }
        }
    }
}

#[component]
pub fn SubmitButton<'a>(cx: Scope<'a>, label: &'a str) -> Element<'a> {
    render! {
            input{
            r#type: "submit",
            font_family: "inter",
            font_size: 16,
            font_weight: 400,
            value: *label,
            }
    }
}
