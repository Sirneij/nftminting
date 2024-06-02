use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// The main function to start the backend server.
/// It will read the settings from the `.env` file and start the application.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    // Determine which .env file to load based on the environment
    if cfg!(debug_assertions) {
        let env_path_dev = std::path::PathBuf::from(".env.dev");
        dotenvy::from_path(env_path_dev.as_path()).ok();
    }

    let settings = backend::settings::get_settings().expect("Failed to read settings.");

    let subscriber_format = if settings.debug {
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=debug,tower_http=debug,axum::rejection=debug,sqlx=debug".into()
        })
    } else {
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=info,tower_http=info,axum::rejection=info,sqlx=info".into()
        })
    };

    tracing_subscriber::registry()
        .with(subscriber_format)
        .with(tracing_subscriber::fmt::layer())
        .init();

    backend::startup::Application::build(settings, None).await?;

    Ok(())
}
