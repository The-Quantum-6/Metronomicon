use openidconnect::{
    ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, RedirectUrl,
    core::{CoreClient, CoreProviderMetadata},
    reqwest::Client,
};
use sqlx::{Pool, Postgres};

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
}

impl AppState {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        let client_id: String =
            std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
        let client_secret: String =
            std::env::var("FEIDE_SECRET").expect("FEIDE_SECRET must be set");
        let redirect_uri: String =
            std::env::var("FEIDE_REDIRECT_URI").expect("FEIDE_REDIRECT_URI must be set");

        let http_client: Client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new("https://auth.dataporten.no".to_string()).unwrap(),
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
        }
    }
}
