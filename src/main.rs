use convert_case::{Case, Casing};
use poem::handler;
use poem::web::{Json, Query};
use poem::{error::NotFoundError, http::StatusCode, middleware::Tracing};
use poem::{get, listener::TcpListener, EndpointExt, Response, Route, Server};
use serde::Deserialize;
use serde_json::json;
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

    let app = Route::new()
        .at("/helloworld", get(helloworld_handler))
        .at("/versionz", get(versionz_handler))
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

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

#[handler]
async fn helloworld_handler(Query(QueryParams { name }): Query<QueryParams>) -> String {
    let mut name_var = "Stranger".to_string();
    if name.is_some() && !name.to_owned().unwrap().is_empty() {
        name_var = name.unwrap().to_case(Case::Title);
    }
    return "Hello ".to_owned()+&name_var;
}

#[handler]
async fn versionz_handler() -> Json<serde_json::Value> {
    return Json(json!({"name":env!("CARGO_PKG_NAME"),"rev":env!("GIT_SHA")}));
}
