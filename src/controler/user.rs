#![allow(non_snake_case)]

use crate::controler::connection::{ConnectionUrls, Token};
use crate::model::users::user::User;
use crate::model::users::user_login::UserLogin;
use crate::model::users::user_role::UserRole;

use bincode::{config, decode_from_slice};

pub async fn login(user: &UserLogin) -> Result<User, anyhow::Error> {
    let config = config::standard();
    let response = reqwest::Client::new()
        .post(ConnectionUrls::Login.to_string())
        .json(user)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::Error::msg("Invalid credentials"));
    }

    let decode = decode_from_slice(&response.bytes().await?, config);

    if decode.is_err() {
        return Err(anyhow::Error::msg("Decoding error"));
    }
    let (decoded, _) = decode?;
    Ok(decoded)
}

pub async fn get_users(token: Token) -> Result<Vec<UserRole>, anyhow::Error> {
    let config = config::standard();
    let response = reqwest::Client::new()
        .post(ConnectionUrls::SelectUsers.to_string())
        .json(&token)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::Error::msg("Invalid credentials"));
    }

    let decode = decode_from_slice(&response.bytes().await?, config);

    if decode.is_err() {
        return Err(anyhow::Error::msg("Decoding error"));
    }
    let (decoded, _) = decode?;
    Ok(decoded)
}

#[derive(serde::Serialize)]
pub(crate) struct UserToken {
    pub(crate) Token: String,
    pub(crate) User: UserRole,
}

pub async fn create_user(user: UserToken) -> Result<UserRole, anyhow::Error> {
    let config = config::standard();
    let response = reqwest::Client::new()
        .post(ConnectionUrls::CreateUser.to_string())
        .json(&user)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::CREATED {
        return Err(anyhow::Error::msg("Invalid credentials"));
    }

    let decode = decode_from_slice(&response.bytes().await?, config);
    match decode {
        Ok((decoded, _)) => Ok(decoded),
        Err(_) => Err(anyhow::Error::msg("Decoding error")),
    }
}
pub async fn update_user(user: UserToken) -> Result<(), anyhow::Error> {
    let response = reqwest::Client::new()
        .post(ConnectionUrls::UpdateUser.to_string())
        .json(&user)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::Error::msg("Invalid credentials"));
    }

    Ok(())
}
pub async fn delete_user(user: UserToken) -> Result<(), anyhow::Error> {
    let response = reqwest::Client::new()
        .delete(ConnectionUrls::DeleteUser.to_string())
        .json(&user)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::Error::msg("Invalid credentials"));
    }

    Ok(())
}
