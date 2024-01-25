pub(crate) struct UserRole {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) role: String,
}

impl UserRole {
    pub(crate) fn new() -> UserRole {
        UserRole {
            username: String::new(),
            password: String::new(),
            role: String::new(),
        }
    }
}
