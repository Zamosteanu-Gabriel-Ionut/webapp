use axum::{routing::{get, post}, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/home", get(handlers::get_posts))
        .route("/create_post", post(handlers::create_post))
}
