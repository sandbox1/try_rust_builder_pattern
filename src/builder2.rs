use crate::User;

#[derive(Default)]
pub struct UserBuilder2 {
    username: Option<String>,
    email: Option<String>,
    sign_in_count: Option<u64>,
    active: Option<bool>,
}

impl UserBuilder2 {
    pub fn new() -> Self {
        Self::default()
    }

    // implicit reference to self instead of &self
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn sign_in_count(mut self, sign_in_count: u64) -> Self {
        self.sign_in_count = Some(sign_in_count);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn build(self) -> User {
        User {
            username: self.username.unwrap_or_default(),
            email: self.email.unwrap_or_default(),
            sign_in_count: self.sign_in_count.unwrap_or_default(),
            active: self.active.unwrap_or_default(),
        }
    }
}
