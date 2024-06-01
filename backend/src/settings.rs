/// Application's specific settings to expose `port`,
/// `host`, `protocol`, and possible url of the application
/// during and after development
#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

/// Secret settings to expose `token_expiration` and `cookie_expiration`
/// for the application
/// `token_expiration` is the time in seconds for the token to expire
/// `cookie_expiration` is the time in seconds for the cookie to expire
/// after the user logs in
#[derive(serde::Deserialize, Clone)]
pub struct Secret {
    pub token_expiration: i64,
    pub cookie_expiration: i64,
}

/// Server email settings to expose `host`, `host_user`, and `host_user_password`
/// for the application to send emails
/// `host` is the email server host
/// `host_user` is the email server user
/// `host_user_password` is the email server user password
#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub host: String,
    pub host_user: String,
    pub host_user_password: String,
}

/// Storage settings to expose `pool_max_open`, `pool_max_idle`, `pool_timeout_seconds`, and `pool_expire_seconds`
/// for the application to connect to the storage server
/// `pool_max_open` is the maximum number of open connections to the storage server
/// `pool_max_idle` is the maximum number of idle connections to the storage server
/// `pool_timeout_seconds` is the time in seconds for the pool to timeout
/// `pool_expire_seconds` is the time in seconds for the pool to expire
#[derive(serde::Deserialize, Clone)]
pub struct StorageSettings {
    pub pool_max_open: u32,
    pub pool_max_idle: u32,
    pub pool_timeout_seconds: u64,
    pub pool_expire_seconds: u64,
}

/// Global settings for the exposing all preconfigured variables
/// for the application
/// `application` is the application settings
/// `debug` is the debug mode for the application
/// `email` is the email settings for the application
/// `frontend_url` is the frontend url for the application
/// `secret` is the secret settings for the application
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub debug: bool,
    pub email: EmailSettings,
    pub frontend_url: String,
    pub secret: Secret,
    pub storage: StorageSettings,
}

/// The possible runtime environment for our application.
pub enum Environment {
    Dev,
    Prod,
}

/// Implement the `Environment` enum to return a string representation
/// of the environment
/// This is useful when we want to print the environment
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
        }
    }
}

/// Implement the `TryFrom` trait to convert a string to an `Environment` enum
/// This is useful when we want to parse the environment variable
/// and convert it to the `Environment` enum
/// This will return an error if the string is not `dev` or `prod`
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a supported environment. Use either `dev` or `prod`.",
                other
            )),
        }
    }
}

/// Get the settings from the configuration files.
///
/// The settings are in the `settings` directory which is in the root of the project
/// And the settings are in the `base.yml` and `dev.yml` or `prod.yml` files
/// The environment specific file is determined by the `APP_ENVIRONMENT` environment variable
/// The value of the `APP_ENVIRONMENT` environment variable can either be `dev` or `prod`
///
/// APP_ENVIRONMENT = dev | prod.
///
/// This will also add the settings from the environment variables
/// with the prefix `APP` and separator `__`
/// E.g.
/// 1. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
/// 2. `APP_DEBUG=true` would set `Settings.debug`
/// 3. `APP_EMAIL__HOST=smtp.gmail.com` would set `Settings.email.host`
pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let settings_directory = base_path.join("settings");

    // Detect the running environment.
    // Default to `dev` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(settings_directory.join("base.yml")))
        .add_source(config::File::from(
            settings_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
