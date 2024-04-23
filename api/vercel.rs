use student_halls_backend::setup_server;
use vercel_runtime::{ process_request, process_response, run_service, Error, ServiceBuilder };
use dotenv::dotenv;
mod lambda;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let app = setup_server().await;

    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .layer(lambda::LambdaLayer::default())
        .service(app);

    run_service(handler).await
}
