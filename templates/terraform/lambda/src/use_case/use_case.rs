use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_http::Body;
use lambda_runtime::LambdaEvent;

use serde::{self, Deserialize, Serialize};

use crate::handler::CommonErrorResponseBody;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleRequestObject {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleResponseBody {
    pub(crate) email: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum SampleUseCaseError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("InternalServerError")]
    InternalServerError,
    #[error("TooManyRequest")]
    TooManyRequest,
    #[error("Unknown error")]
    Unknown,
}

impl SampleRequestObject {
    pub(crate) fn from_event(
        event: &LambdaEvent<ApiGatewayProxyRequest>,
    ) -> Result<Self, SampleUseCaseError> {
        let body = match &event.payload.body {
            Some(v) => v,
            None => return Err(SampleUseCaseError::InvalidRequest),
        };
        match serde_json::from_str::<SampleRequestObject>(body) {
            Ok(val) => Ok(val),
            Err(_err) => Err(SampleUseCaseError::InvalidRequest),
        }
    }
}

impl From<SampleUseCaseError> for ApiGatewayProxyResponse {
    fn from(val: SampleUseCaseError) -> Self {
        match val {
            SampleUseCaseError::InvalidRequest => {
                let error_response_body = CommonErrorResponseBody {
                    error_code: 0,
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
            SampleUseCaseError::TooManyRequest => {
                let error_response_body = CommonErrorResponseBody {
                    error_code: 1,
                    error_message: "Too many request".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 429,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
            SampleUseCaseError::InternalServerError => {
                let error_response_body = CommonErrorResponseBody {
                    error_code: 2,
                    error_message: "Internal server error".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 500,
                    headers: headers.clone(),
                    multi_value_headers: headers.clone(),
                    body: Some(Body::Text(
                        serde_json::to_string(&error_response_body).unwrap(),
                    )),
                    is_base64_encoded: false,
                }
            }
            SampleUseCaseError::Unknown => {
                let error_response_body = CommonErrorResponseBody {
                    error_code: 1_000,
                    error_message: "InternalServerError".to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                ApiGatewayProxyResponse {
                    status_code: 500,
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

pub(crate) async fn call(
    _aws_config: &aws_config::SdkConfig,
    event: &LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Box<dyn std::error::Error>> {
    // construct request content from lambda event.
    let _request_object = match SampleRequestObject::from_event(event) {
        Ok(v) => v,
        Err(err) => return Ok(err.into()),
    };
    // TODO: do something
    // construct response
    let response_body = SampleResponseBody {
        email: "sample@example.com".to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: headers.clone(),
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(Body::Text(serde_json::to_string(&response_body).unwrap())),
    })
}
