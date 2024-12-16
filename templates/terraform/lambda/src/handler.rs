use crate::use_case;
use aws_config::BehaviorVersion;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_http::Body;
use lambda_runtime::LambdaEvent;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommonErrorResponseBody {
    pub(crate) error_code: i32,
    pub(crate) error_message: String,
}

pub(crate) async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, lambda_runtime::Error> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    // 必要であれば、use caseを選択してそれを呼び出すようにする
    let response = match use_case::{{ use_case_name|snake }}_use_case::call(&aws_config, &event).await {
        Ok(val) => val,
        Err(err) => {
            warn!("{:?}", err);
            let headers = HeaderMap::new();
            ApiGatewayProxyResponse {
                status_code: 500,
                headers: headers.clone(),
                multi_value_headers: headers.clone(),
                body: Some(Body::Text("Unhandled Error Occurred".to_string())),
                is_base64_encoded: false,
            }
        }
    };
    Ok(response)
}
