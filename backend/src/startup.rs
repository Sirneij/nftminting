use crate::routes;
use axum::extract::FromRef;
use tokio::signal;

pub struct Application {
    port: u16,
}

#[derive(Clone)]
pub struct AppState {
    pub db_store: crate::store::Store,
    pub redis_store: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
    key: axum_extra::extract::cookie::Key,
}

impl FromRef<AppState> for axum_extra::extract::cookie::Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl Application {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn build(
        settings: crate::settings::Settings,
        test_pool: Option<sqlx::postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let store = if let Some(pool) = test_pool {
            crate::store::Store { connection: pool }
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            crate::store::Store::new(&db_url, &settings).await
        };

        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Failed to migrate");

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tracing::info!("Listening on {}", &address);

        run(listener, store, settings).await;

        Ok(Self { port })
    }
}

async fn run(
    listener: tokio::net::TcpListener,
    store: crate::store::Store,
    settings: crate::settings::Settings,
) {
    let redis_url = std::env::var("REDIS_URL").expect("Failed to get REDIS_URL.");
    let manager =
        bb8_redis::RedisConnectionManager::new(redis_url).expect("Failed to create redis manager");
    let redis_pool = bb8_redis::bb8::Pool::builder()
        .max_size(settings.storage.pool_max_open)
        .min_idle(settings.storage.pool_max_idle)
        .connection_timeout(std::time::Duration::from_secs(
            settings.storage.pool_timeout_seconds,
        ))
        .build(manager)
        .await
        .expect("Failed to create redis pool.");

    let cors = tower_http::cors::CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![
            axum::http::Method::OPTIONS,
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
        ])
        .allow_headers(vec![
            axum::http::header::ORIGIN,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_origin(
            settings
                .frontend_url
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        );

    let app_state = AppState {
        db_store: store,
        redis_store: redis_pool,
        key: axum_extra::extract::cookie::Key::from(
            std::env::var("COOKIE_SECRET")
                .expect("Failed to get COOKIE_SECRET.")
                .as_bytes(),
        ),
    };
    // build our application with a route
    let app = axum::Router::new()
        .route(
            "/api/health-check",
            axum::routing::get(routes::health_check),
        )
        .with_state(app_state.clone())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
