use chrono::{DateTime, Utc};
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Profile {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
    pub banner_url: Option<String>,
    pub visibility: ProfileVisibility,
    pub favorite_genres: Option<String>,
    pub reading_since: Option<NaiveDate>,
    pub followers_count: i32,
    pub friends_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileVisibility {
    Public,
    Private,
}


impl Profile {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            bio: None,
            pronouns: None,
            location: None,
            banner_url: None,
            visibility: ProfileVisibility::Public,
            favorite_genres: None,
            reading_since: None,
            followers_count: 0,
            friends_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn is_private(&self) -> bool {
        matches!(self.visibility, ProfileVisibility::Private)
    }

    pub fn is_public(&self) -> bool {
        self.visibility == ProfileVisibility::Public
    }

    pub fn is_owner(&self, user_id: &Uuid) -> bool {
        self.user_id == *user_id
    }

    pub fn can_view(&self, viewer_id: &Uuid) -> bool {
        self.is_public() || self.is_owner(viewer_id)
    }

    pub fn update_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }

    pub fn update_pronouns(&mut self, pronouns: Option<String>) {
        self.pronouns = pronouns;
    }

    pub fn update_location(&mut self, location: Option<String>) {
        self.location = location;
    }

    pub fn update_banner_url(&mut self, banner_url: Option<String>) {
        self.banner_url = banner_url;
    }

    pub fn update_visibility(&mut self, visibility: ProfileVisibility) {
        self.visibility = visibility;
    }

    pub fn update_favorite_genres(&mut self, favorite_genres: Option<String>) {
        self.favorite_genres = favorite_genres;
    }

    pub fn update_reading_since(&mut self, reading_since: Option<NaiveDate>) {
        self.reading_since = reading_since;
    }

    pub fn update_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn update_followers_count(&mut self, followers_count: i32) {
        self.followers_count = followers_count;
    }

    pub fn update_friends_count(&mut self, friends_count: i32) {
        self.friends_count = friends_count;
    }

    pub fn increment_friends_count(&mut self) {
        self.friends_count += 1;
    }

    pub fn decrement_friends_count(&mut self) {
        self.friends_count -= 1;
    }
}