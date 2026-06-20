use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::container::Container;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Book Vally API",
        version = "0.1.0",
        description = "Social reading app — bring-your-own book files, read or listen, track streaks, share with friends.",
        contact(name = "Book Vally", email = "kolawoleoluwasegun567@gmail.com"),
        license(name = "MIT")
    ),
    tags(
        (name = "auth",         description = "OAuth login, JWT refresh, session management, device registration"),
        (name = "catalog",      description = "Books, file import from cloud, parsing"),
        (name = "library",      description = "User shelf, reading status, queue"),
        (name = "reader",       description = "Reading progress, highlights, bookmarks"),
        (name = "tts",          description = "Text-to-speech voices and playback"),
        (name = "streaks",      description = "Daily activity, streak freezes, rollover"),
        (name = "gamification", description = "XP, achievements, leagues"),
        (name = "plans",        description = "Reading plans, groups, progress"),
        (name = "discover",     description = "Search, trending, friends reading now"),
        (name = "social",       description = "Friends, invites, notifications, push"),
        (name = "stats",        description = "Dashboard and analytics projections"),
        (name = "profile",      description = "Profile, settings, reminders")
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development"),
    )
)]
pub struct ApiDoc;

pub fn all_routes(container: Arc<Container>) -> axum::Router {
    use http_kit::{request_id_layer, trace_layer};

    let auth_state = feat_auth::wiring::AuthState::new(
        container.db.clone(),
        container.jwt.clone(),
        container.config.auth.refresh_ttl_secs as i64,
        container.config.auth.access_ttl_secs,
        container.redis.clone(),
        container.kafka.clone(),
    );

    let profile_state = feat_profile::wiring::ProfileState::new(
        container.db.clone(),
        container.jwt.clone(),
    );

    let mut openapi = ApiDoc::openapi();
    openapi.merge(feat_auth::http::AuthApiDoc::openapi());
    openapi.merge(feat_profile::http::ProfileApiDoc::openapi());

    openapi.components = Some({
        let mut c = openapi.components.take().unwrap_or_default();
        c.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        c
    });
    // openapi.merge(feat_catalog::http::CatalogApiDoc::openapi());  — add as features are built

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", openapi))
        .merge(feat_auth::http::routes::routes().with_state(auth_state))
        .merge(feat_profile::http::routes::routes().with_state(profile_state))

        .layer(trace_layer())
        .layer(request_id_layer())
        .with_state(container)
}
