use chrono::NaiveDate;
use uuid::Uuid;
use kernel::AppError;
use crate::application::ports::ProfileRepository;
use crate::domain::profile::{Profile, ProfileVisibility};

pub struct UpdateProfileInput {
    pub bio:             Option<String>,
    pub pronouns:        Option<String>,
    pub location:        Option<String>,
    pub banner_url:      Option<String>,
    pub visibility:      Option<String>,
    pub favorite_genres: Option<String>,
    pub reading_since:   Option<String>,
}

pub struct UpdateProfile<R> { pub repository: R }

impl<R: ProfileRepository> UpdateProfile<R> {
    pub async fn execute(&self, user_id: Uuid, input: UpdateProfileInput) -> Result<Profile, AppError> {
        let mut profile = self.repository
            .find_profile(user_id)
            .await
            .map_err(AppError::internal)?
            .unwrap_or_else(|| Profile::new(user_id));

        if let Some(bio) = input.bio { profile.update_bio(Some(bio)); }
        if let Some(p) = input.pronouns { profile.update_pronouns(Some(p)); }
        if let Some(l) = input.location { profile.update_location(Some(l)); }
        if let Some(b) = input.banner_url { profile.update_banner_url(Some(b)); }
        if let Some(g) = input.favorite_genres { profile.update_favorite_genres(Some(g)); }

        if let Some(v) = input.visibility {
            let vis = match v.as_str() {
                "public"  => ProfileVisibility::Public,
                "private" => ProfileVisibility::Private,
                other => return Err(AppError::UnprocessableEntity(
                    format!("invalid visibility: {other}")
                )),
            };
            profile.update_visibility(vis);
        }

        if let Some(s) = input.reading_since {
            let date = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map_err(|_| AppError::UnprocessableEntity(
                    "invalid reading_since format, use YYYY-MM-DD".into()
                ))?;
            profile.update_reading_since(Some(date));
        }

        profile.update_updated_at();

        self.repository.upsert_profile(&profile).await.map_err(AppError::internal)?;

        Ok(profile)
    }
}
