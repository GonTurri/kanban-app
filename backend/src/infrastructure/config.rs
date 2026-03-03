use std::env;

use std::time::Duration;

const DEFAULT_REFRESH_TOKEN_TTL_DAYS: &str = "30";
const DEFAULT_ACCESS_TOKEN_TTL_MINS: &str = "15";

pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub jwt_secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

impl AppConfig {
    pub fn from_env() -> Self {

        let host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_owned());
        let port = env::var("SERVER_PORT").unwrap_or("8080".to_owned());

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let refresh_token_ttl_days: u64 = env::var("REFRESH_TOKEN_TTL_DAYS")
            .unwrap_or(DEFAULT_REFRESH_TOKEN_TTL_DAYS.to_string())
            .parse()
            .expect("REFRESH_TOKEN_TTL_DAYS must be a number");

        let access_token_ttl_mins: u64 = env::var("ACCESS_TOKEN_TTL_MINS")
            .unwrap_or(DEFAULT_ACCESS_TOKEN_TTL_MINS.to_string())
            .parse()
            .expect("ACCESS_TOKEN_TTL_MINS must be a valid number");

        Self {
            host,
            port,
            jwt_secret,
            access_token_ttl: Duration::from_secs(access_token_ttl_mins * 60),
            refresh_token_ttl: Duration::from_secs(refresh_token_ttl_days * 24 * 60 * 60),
        }
    }
}