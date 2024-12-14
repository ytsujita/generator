pub mod sample_use_case;
use aws_config::BehaviorVersion;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_http::Body;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MyApiError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("InternalServerError")]
    InternalServerError,
    #[error("TooManyRequest")]
    TooManyRequest,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponseBody {
    pub(crate) error_code: i32,
    pub(crate) error_message: String,
}

pub(crate) async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, lambda_runtime::Error> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    match sample_use_case::call(&aws_config, &event).await {
        Ok(res) => Ok(res),
        Err(err) => Ok(err.into()),
    }
}

impl From<MyApiError> for ApiGatewayProxyResponse {
    fn from(val: MyApiError) -> Self {
        match val {
            MyApiError::InvalidRequest => {
                let error_response_body = ErrorResponseBody {
                    error_code: 1_000,
                    error_message: "Invalid Request".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
            MyApiError::InternalServerError => {
                let error_response_body = ErrorResponseBody {
                    error_code: 1_000,
                    error_message: "sample".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
            MyApiError::TooManyRequest => {
                let error_response_body = ErrorResponseBody {
                    error_code: 1_000,
                    error_message: "sample".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
            MyApiError::Unknown => {
                let error_response_body = ErrorResponseBody {
                    error_code: 1_000,
                    error_message: "sample".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 400,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
        }
    }
}
