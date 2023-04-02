use mailcolobus::configuration::get_configuration;
use mailcolobus::startup::run;
use mailcolobus::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initiliaze log subscriber for telemetry
    let subscriber = get_subscriber("mailcolubus".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't get our configs
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failec to connect to Postgres");

    // Start listener
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    tracing::info!("Starting on port {}", port);
    run(listener, connection)?.await
}
