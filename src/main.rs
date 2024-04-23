use student_halls_backend::setup_server;
use tracing_subscriber;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = setup_server().await;
    let addr = "0.0.0.0:8080".parse().unwrap();
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
