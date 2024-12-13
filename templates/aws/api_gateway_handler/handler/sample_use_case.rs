use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_http::Body;
use lambda_runtime::LambdaEvent;

use serde::{self, Deserialize, Serialize};

use super::MyApiError;
use crate::dynamodb::sample_query;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleRequestObject {
    pub(crate) email: String,
    pub(crate) password: String,
}

impl SampleRequestObject {
    pub(crate) fn from_event(
        event: &LambdaEvent<ApiGatewayProxyRequest>,
    ) -> Result<Self, MyApiError> {
        let body = match &event.payload.body {
            Some(v) => v,
            None => return Err(MyApiError::InvalidRequest),
        };
        match serde_json::from_str::<SampleRequestObject>(body) {
            Ok(val) => Ok(val),
            Err(_err) => Err(MyApiError::InvalidRequest),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleResponseBody {
    pub(crate) email: String,
}

pub(crate) async fn call(
    aws_config: &aws_config::SdkConfig,
    event: &LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, MyApiError> {
    // construct request content from lambda event.
    let request_object = match SampleRequestObject::from_event(event) {
        Ok(val) => val,
        Err(_err) => return Err(MyApiError::InvalidRequest),
    };
    // TODO: do something
    let sample_response_body = sample_query(aws_config, &request_object).await?;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: headers.clone(),
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(Body::Text(
            serde_json::to_string(&sample_response_body).unwrap(),
        )),
    })
}
