use code_challenge::router::app_router;
use poem::{
    EndpointExt,
    error::NotFoundError,
    http::StatusCode,
    listener::TcpListener,
    middleware::Tracing, Response, Server,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=info");
    }
    //Setup logging & tracing
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = app_router()
        .catch_error(|_err: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("not found")
        })
        .with(Tracing);

    let port = std::env::var("PORT");
    let port = match port {
        Ok(port) => port,
        Err(_) => "8080".to_string(),
    };

    Server::new(TcpListener::bind("0.0.0.0:".to_owned() + port.as_str()))
        .name("code-challenge")
        .run(app)
        .await
}
