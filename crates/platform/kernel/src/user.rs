use serde::{Deserialize, Serialize};
use crate::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthUser {
    id: UserId,
    handle: String,
}

impl AuthUser {
    pub fn new(id: UserId, handle: String) -> Self {
        Self { id, handle }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn handle(&self) -> &str {
        &self.handle
    }
}