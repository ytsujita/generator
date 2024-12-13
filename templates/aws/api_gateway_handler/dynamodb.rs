use crate::handler::MyApiError;

use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use serde::{Deserialize, Serialize};

use super::handler::sample_use_case::SampleRequestObject;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleTableItem {
    attr_01_name: String,
    attr_02_name: String,
    attr_03_name: String,
}

pub(crate) async fn sample_query(
    aws_config: &aws_config::SdkConfig,
    query_object: &SampleRequestObject,
) -> Result<Vec<SampleTableItem>, MyApiError> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var("foobar").unwrap();
    let query_result = client
        .query()
        .table_name(&sample_table_name)
        .key_condition_expression("#pk_name = :pk_val")
        .key_condition_expression("#sk_name = :sk_val")
        .expression_attribute_names("#pk_name", "PK")
        .expression_attribute_names("#sk_name", "SK")
        .expression_attribute_values(":pk_val", AttributeValue::N(query_object.email.clone()))
        .expression_attribute_values(":sk_val", AttributeValue::N(query_object.password.clone()))
        .send()
        .await;
    match query_result {
        Ok(val) => {
            let mut items: Vec<SampleTableItem> = vec![];
            for item in val.items().iter() {
                let _value = item
                    .iter()
                    .map(|f| f.1.as_s().unwrap().as_str().to_string())
                    .collect::<Vec<String>>();
                items.push(
                    SampleTableItem {
                        attr_01_name: "foobar".to_string(),
                        attr_02_name: "foobar".to_string(),
                        attr_03_name: "foobar".to_string(),
                    });
            }
            Ok(items)
        },
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::query::QueryError::InternalServerError(_) => Err(MyApiError::InternalServerError),
            aws_sdk_dynamodb::operation::query::QueryError::InvalidEndpointException(_) => Err(MyApiError::InternalServerError),
            aws_sdk_dynamodb::operation::query::QueryError::ProvisionedThroughputExceededException(_) => Err(MyApiError::TooManyRequest),
            aws_sdk_dynamodb::operation::query::QueryError::RequestLimitExceeded(_) => Err(MyApiError::TooManyRequest),
            aws_sdk_dynamodb::operation::query::QueryError::ResourceNotFoundException(_) => Err(MyApiError::InternalServerError),
            err if err.code() == Some("Unhandled") => Err(MyApiError::Unknown),
            _err => Err(MyApiError::Unknown),
        },
    }
}

// pub(crate) async fn get_item(client: &Client) -> Result<ResponseItem, MyApiError> {
//     let res = client
//         .get_item()
//         .table_name(SAMPLE_TABLE_NAME)
//         .key("", AttributeValue::S("".to_string()))
//         .send()
//         .await;
//     match res {
//         Ok(_val) => todo!(),
//         Err(err) => match err.into_service_error() {
//             aws_sdk_dynamodb::operation::get_item::GetItemError::InternalServerError => todo!(),
//             aws_sdk_dynamodb::operation::get_item::GetItemError::InvalidEndpointException(_) => todo!(),
//             aws_sdk_dynamodb::operation::get_item::GetItemError::ProvisionedThroughputExceededException(_) => todo!(),
//             aws_sdk_dynamodb::operation::get_item::GetItemError::RequestLimitExceeded(_) => todo!(),
//             aws_sdk_dynamodb::operation::get_item::GetItemError::ResourceNotFoundException(_) => todo!(),
//             err if err.code() == Some("Unhandled") => todo!(),
//             err => return MyApiError::Unknown,
//         },
//     };
//     let res = client
//         .put_item()
//         .table_name(SAMPLE_TABLE_NAME)
//         .item("sample", AttributeValue::S("sample".to_string()))
//         .send()
//         .await;
//     match res {
//         Ok(_) => todo!(),
//         Err(err) => match err.into_service_error() {
//             aws_sdk_dynamodb::operation::put_item::PutItemError::ConditionalCheckFailedException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::InternalServerError => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::InvalidEndpointException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::ItemCollectionSizeLimitExceededException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::ProvisionedThroughputExceededException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::ReplicatedWriteConflictException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::RequestLimitExceeded(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::ResourceNotFoundException(_) => todo!(),
//             aws_sdk_dynamodb::operation::put_item::PutItemError::TransactionConflictException(_) => todo!(),
//             err if err.code() == Some("Unhandled") => todo!(),
//             err => return MyApiError::Unknown,
//         },
//     }
//     let res = client
//         .update_item()
//         .table_name(SAMPLE_TABLE_NAME)
//         .key(k, v)
//         .update_expression(input)
//         .expression_attribute_names(k, v)
//         .expression_attribute_names(k, v)
//         .expression_attribute_values(k, v)
//         .expression_attribute_values(k, v)
//         .send()
//         .await;
//     match res {
//         Ok(_) => todo!(),
//         Err(err) => match err.into_service_error() {
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::ConditionalCheckFailedException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::InternalServerError => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::InvalidEndpointException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::ItemCollectionSizeLimitExceededException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::ProvisionedThroughputExceededException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::ReplicatedWriteConflictException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::RequestLimitExceeded(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::ResourceNotFoundException(_) => todo!(),
//             aws_sdk_dynamodb::operation::update_item::UpdateItemError::TransactionConflictException(_) => todo!(),
//             err if err.code() == Some("Unhandled") => todo!(),
//             err => return MyApiError::Unknown,
//         },
//     }
// }
//
// pub(crate) async fn sample_command(
//     client: &Client,
// ) -> Result<ResponseItem, Box<dyn std::error::Error>> {
//     let put_01_action = aws_sdk_dynamodb::types::Put::builder()
//         .table_name(SAMPLE_TABLE_NAME)
//         .item("", AttributeValue::S("".to_string()))
//         .build()
//         .unwrap();
//     let put_01_transact = aws_sdk_dynamodb::types::TransactWriteItem::builder()
//         .put(put_01_action)
//         .build();
//     let put_result = client
//         .transact_write_items()
//         .transact_items(put_01_transact)
//         .transact_items(put_01_transact)
//         .send()
//         .await?;
//     Ok(ResponseItem {
//         email: "email".to_string(),
//         password: "password".to_string(),
//     })
// }
