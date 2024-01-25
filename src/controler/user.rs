use crate::controler::connection::ConnectionUrls;
use crate::model::users::user::User;
use crate::model::users::user_login::UserLogin;
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
