use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

pub fn statics_route_handler() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}
