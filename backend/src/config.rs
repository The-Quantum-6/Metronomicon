pub struct AppConfig {
    pub cors_should_be_permissive: bool,
    pub database_url: String,

}

pub fn get() -> AppConfig {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    AppConfig {
        cors_should_be_permissive: environment == "dev",
        database_url
    }
}