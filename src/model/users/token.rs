#![allow(non_snake_case)]
use serde::Serialize;

#[derive(Serialize)]
struct Token {
    Token: String,
}

impl Token {
    pub fn new() -> Token {
        Token {
            Token: String::new(),
        }
    }
    pub fn from(token: String) -> Token {
        Token { Token: token }
    }

    pub fn set_token(&mut self, token: &String) {
        self.Token = token.to_string();
    }

    pub fn get_token(&self) -> &String {
        &self.Token
    }
}
