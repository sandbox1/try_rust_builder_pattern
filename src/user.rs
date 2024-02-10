#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

// Associated Function
impl User {
    pub fn new(email: String, username: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }
}

// Default trait
impl Default for User {
    fn default() -> Self {
        Self {
            username: String::from("default"),
            email: String::from("unkown"),
            sign_in_count: 0,
            active: false,
        }
    }
}
