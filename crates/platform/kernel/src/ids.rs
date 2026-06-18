use std::str::FromStr;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Clone, PartialEq, Eq, Copy, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() ->
        Self { Self(Uuid::new_v4()) }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}