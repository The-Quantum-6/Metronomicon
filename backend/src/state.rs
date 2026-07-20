use jsonwebtoken::{DecodingKey, EncodingKey};
use openidconnect::{
    ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, RedirectUrl,
    core::{CoreClient, CoreProviderMetadata},
    reqwest::Client,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::storage::Storage;
use axum::extract::FromRef;

pub type OidcClient = CoreClient<
    EndpointSet,      // HasAuthUrl
    EndpointNotSet,   // HasDeviceAuthUrl
    EndpointNotSet,   // HasIntrospectionUrl
    EndpointNotSet,   // HasRevocationUrl
    EndpointMaybeSet, // HasTokenUrl
    EndpointMaybeSet, // HasUserInfoUrl (adjust if you set it)
>;

#[derive(Clone)]
pub struct AppState {
    pub oidc_client: OidcClient,
    pub http_client: reqwest::Client,
    pub pool: Pool<Postgres>,
    pub storage: Storage,
    pub jwt_encode: Arc<EncodingKey>,
    pub jwt_decode: Arc<DecodingKey>,
}

impl AppState {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        let client_id: String =
            std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        let client_secret: String =
            std::env::var("GOOGLE_SECRET").expect("GOOGLE_SECRET must be set");
        let redirect_uri: String =
            std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");

        let jsonwebtoken_secret: String =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let storage = Storage::from_env().await;

        let http_client: Client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new("https://accounts.google.com".to_string()).unwrap(),
            &http_client,
        )
        .await
        .expect("Provider metadata should be discoverable");

        let oidc_client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());
        AppState {
            oidc_client,
            http_client,
            pool,
            storage,
            jwt_encode: Arc::new(EncodingKey::from_secret(jsonwebtoken_secret.as_ref())),
            jwt_decode: Arc::new(DecodingKey::from_secret(jsonwebtoken_secret.as_ref())),
        }
    }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Self {
        state.storage.clone()
    }
}
