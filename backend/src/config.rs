pub struct AppConfig {
    pub cors_should_be_permissive: bool,
    pub database_url: String,
    pub frontend_url: String,
    pub karakterweb_api_key: String,
}

pub fn get() -> AppConfig {
    dotenvy::dotenv().ok();
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let karakterweb_api_key =
        std::env::var("KARAKTERWEB_API_KEY").expect("KARAKTERWEB_API_KEY must be set");
    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    AppConfig {
        cors_should_be_permissive: environment == "dev",
        database_url,
        frontend_url,
        karakterweb_api_key,
    }
}
