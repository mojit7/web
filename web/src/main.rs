//! Shop-Proof Web Server
//!
//! This is the main entry point for the web server.
//! It uses Axum for HTTP routing and HTMX for dynamic frontend updates.

use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router
    let app = Router::new()
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        // Routes
        .route("/", get(routes::index::index))
        .route("/product/:id", get(routes::products::get_product))
        .route("/products", get(routes::products::list_products))
        .route("/cart/add", post(routes::cart::add_to_cart))
        .route("/cart", get(routes::cart::view_cart))
        .route("/cart/remove", post(routes::cart::remove_from_cart))
        .route("/order", post(routes::orders::create_order))
        .route("/order/:id", get(routes::orders::get_order))
        .route("/admin/login", get(routes::admin::login))
        .route("/admin/products", get(routes::admin::list_products))
        .route("/admin/products", post(routes::admin::add_product))
        .route("/admin/stock", post(routes::admin::set_stock));

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Shop-Proof server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
