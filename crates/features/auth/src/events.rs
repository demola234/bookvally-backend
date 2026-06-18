use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const USER_REGISTERED: &str = "auth.user_registered";
pub const USER_LOGGED_IN:  &str = "auth.user_logged_in";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub user_id: Uuid,
    pub handle:  String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    pub user_id: Uuid,
}
