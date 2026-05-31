use {{crate_name}}::{
    routes::health::health_routes,
    state::{AppState, SecretStore},
};
use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let secret_store = SecretStore;

    let app_state = AppState {
        secret_store,
        started_at: std::time::Instant::now(),
    };

    let health_routes = health_routes();
    let app = Router::new()
        .merge(health_routes)
        .with_state(app_state.clone());

    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("API running on: http://localhost:{}/", port);
    axum::serve(listener, app).await.unwrap();
}
