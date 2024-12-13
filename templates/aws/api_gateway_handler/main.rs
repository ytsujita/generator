use lambda_runtime::service_fn;
mod dynamodb;
mod handler;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(handler::handler)).await
}
