mod query;
mod store;
mod sync;

use lambda_http::{
    handler,
    http::StatusCode,
    lambda_runtime::{Context, Error},
    IntoResponse, Request,
};
use serde_json::json;
use sync::{perform_sync, response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::lambda_runtime::run(handler(handle_request)).await?;
    Ok(())
}

async fn handle_request(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    match perform_sync(request).await {
        Ok(value) => response(StatusCode::OK, value),
        Err(error) => response(
            StatusCode::BAD_REQUEST,
            json!({
                "message": error.to_string()
            }),
        ),
    }
}
