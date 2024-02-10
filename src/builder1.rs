use crate::User;

#[derive(Default)]
pub struct UserBuilder1 {
    username: Option<String>,
    email: Option<String>,
    sign_in_count: Option<u64>,
    active: Option<bool>,
}

impl UserBuilder1 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn sign_in_count(&mut self, sign_in_count: u64) -> &mut Self {
        self.sign_in_count = Some(sign_in_count);
        self
    }

    pub fn active(&mut self, active: bool) -> &mut Self {
        self.active = Some(active);
        self
    }

    pub fn build(&self) -> User {
        User {
            username: self
                .username
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            email: self.email.clone().unwrap_or_default(),
            sign_in_count: self.sign_in_count.unwrap_or(0),
            active: self.active.unwrap_or(false),
        }
    }
}
