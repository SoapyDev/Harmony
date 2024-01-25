use serde::Serialize;

#[derive(Serialize)]
struct Token {
    token: String,
}

impl Token{
    pub fn new() -> Token {
        Token {
            token: String::new(),
        }
    }
    pub fn from(token: String) -> Token {
        Token {
            token,
        }
    }
    
    pub fn set_token(&mut self, token: &String) {
        self.token = token.to_string();
    }
    
    pub fn get_token(&self) -> &String {
        &self.token
    }
}