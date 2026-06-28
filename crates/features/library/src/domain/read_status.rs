use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryStatus {
    Queued,
    Reading,
    Finished,
    Dropped,
}

impl LibraryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Reading => "reading",
            Self::Finished => "finished",
            Self::Dropped => "dropped",
        }
    }
}

impl TryFrom<String> for LibraryStatus {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "queued" => Ok(Self::Queued),
            "reading" => Ok(Self::Reading),
            "finished" => Ok(Self::Finished),
            "dropped" => Ok(Self::Dropped),
            _ => Err(anyhow::anyhow!("invalid library_status: {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddedVia {
    Import,
    Discover,
    FriendShare,
}

impl AddedVia {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Import => "import",
            Self::Discover => "discover",
            Self::FriendShare => "friend_share",
        }
    }
}

impl TryFrom<String> for AddedVia {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "import" => Ok(Self::Import),
            "discover" => Ok(Self::Discover),
            "friend_share" => Ok(Self::FriendShare),
            _ => Err(anyhow::anyhow!("invalid added_via: {s}")),
        }
    }
}
