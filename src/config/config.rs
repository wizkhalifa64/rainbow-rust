use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url = env::var("DATABASE_URL").expect("Unable to load env");
        let jwt_secret = env::var("JWT_SECRET").expect("Unable to load env");
        let jwt_expires_in = env::var("JwT_EXPIRED_IN").expect("Unable to load env");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("Unable to load env");
        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}
