use aws_config::SdkConfig;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use log::info;
use serde::{Deserialize, Serialize};

static TABLE_ENV_NAME: &str = "foobar";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SampleTableItem {
    pk_name: String,
    sk_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attr_name: Option<String>,
}

impl SampleTableItem {
    fn new() -> Self {
        SampleTableItem {
            pk_name: "".to_string(),
            sk_name: "".to_string(),
            attr_name: None,
        }
    }
    fn pk_name(&mut self, name: String) -> &mut Self {
        self.pk_name = name;
        self
    }
    fn sk_name(&mut self, name: String) -> &mut Self {
        self.sk_name = name;
        self
    }
    fn attr_name(&mut self, name: Option<String>) -> &mut Self {
        self.attr_name = name;
        self
    }
    fn build(&mut self) -> Self {
        SampleTableItem {
            pk_name: self.pk_name.clone(),
            sk_name: self.sk_name.clone(),
            attr_name: self.attr_name.clone(),
        }
    }
    fn from(value: &std::collections::HashMap<String, AttributeValue>) -> Result<Self, String> {
        let pk_val = value
            .get("PK")
            .ok_or("PK not found")?
            .as_s()
            .expect("Invalid PK type");
        let sk_val = value
            .get("SK")
            .ok_or("SK not found")?
            .as_s()
            .expect("Invalid SK type");
        let attr_val = value
            .get("AttrVal")
            .map(|v| v.as_s().expect("Invalid attr type").to_owned());
        Ok(SampleTableItem {
            pk_name: pk_val.clone(),
            sk_name: sk_val.clone(),
            attr_name: attr_val.clone(),
        })
    }
}

pub(crate) async fn _query(aws_config: &aws_config::SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var("foobar").unwrap();
    let query_result = client
        .query()
        .table_name(&sample_table_name)
        .index_name("sample_index_name")
        .key_condition_expression("#pk_name = :pk_val")
        .key_condition_expression("#sk_name = :sk_val")
        .expression_attribute_names("#pk_name", "PK")
        .expression_attribute_names("#sk_name", "SK")
        .expression_attribute_values(":pk_val", AttributeValue::N("email".to_string()))
        .expression_attribute_values(":sk_val", AttributeValue::N("password".to_string()))
        .send()
        .await;
    match query_result {
        Ok(val) => {
            let mut items: Vec<SampleTableItem> = vec![];
            for item in val.items().iter() {
                if let Ok(parsed_value) = SampleTableItem::from(item) {
                    items.push(parsed_value);
                }
            }
        },
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::query::QueryError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::query::QueryError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::query::QueryError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::query::QueryError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::query::QueryError::ResourceNotFoundException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}

pub(crate) async fn _get_item(aws_config: &SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var(TABLE_ENV_NAME).unwrap();
    let res = client
        .get_item()
        .table_name(&sample_table_name)
        .key("sample", AttributeValue::S("sample".to_string()))
        .send()
        .await;
    match res {
        Ok(val) => {
            info!("{:?}", val);
        },
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::get_item::GetItemError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::get_item::GetItemError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::get_item::GetItemError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::get_item::GetItemError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::get_item::GetItemError::ResourceNotFoundException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}

pub(crate) async fn _put_item(aws_config: &SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var(TABLE_ENV_NAME).unwrap();
    let _sample = SampleTableItem::new()
        .pk_name("sample".to_string())
        .sk_name("sample".to_string())
        .attr_name(Some("sample".to_string()))
        .build();
    let res = client
        .put_item()
        .table_name(&sample_table_name)
        .item("sample", AttributeValue::S("sample".to_string()))
        .send()
        .await;
    match res {
        Ok(_) => {},
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::put_item::PutItemError::ConditionalCheckFailedException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::ItemCollectionSizeLimitExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::ReplicatedWriteConflictException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::ResourceNotFoundException(_) => todo!(),
            aws_sdk_dynamodb::operation::put_item::PutItemError::TransactionConflictException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}

pub(crate) async fn _update_item(aws_config: &SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var(TABLE_ENV_NAME).unwrap();
    let res = client
        .update_item()
        .table_name(&sample_table_name)
        .key("sample", AttributeValue::S("sample".to_string()))
        .update_expression("set #sample = :sample")
        .expression_attribute_names("#sample", "sample")
        .expression_attribute_values(":sample", AttributeValue::S("sample".to_string()))
        .send()
        .await;
    match res {
        Ok(val) => {
            info!("{:?}", val);
        },
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::ConditionalCheckFailedException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::ItemCollectionSizeLimitExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::ReplicatedWriteConflictException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::ResourceNotFoundException(_) => todo!(),
            aws_sdk_dynamodb::operation::update_item::UpdateItemError::TransactionConflictException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}

pub(crate) async fn _transact_write(aws_config: &SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var(TABLE_ENV_NAME).unwrap();
    let put_01_action = aws_sdk_dynamodb::types::Put::builder()
        .table_name(&sample_table_name)
        .item("sample", AttributeValue::S("sample".to_string()))
        .build()
        .unwrap();
    let put_01_transact = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .put(put_01_action)
        .build();
    let put_result = client
        .transact_write_items()
        .transact_items(put_01_transact)
        .send()
        .await;
    match put_result {
        Ok(val) => {
            info!("{:?}", val);
        },
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::IdempotentParameterMismatchException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::ResourceNotFoundException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::TransactionCanceledException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::TransactionInProgressException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}

pub(crate) async fn _transact_get(aws_config: &SdkConfig) -> Result<(), ()> {
    let client = Client::new(aws_config);
    let sample_table_name = dotenvy::var(TABLE_ENV_NAME).unwrap();
    let get_01_action = aws_sdk_dynamodb::types::Get::builder()
        .table_name(&sample_table_name)
        .key("sample", AttributeValue::S("sample".to_string()))
        .build()
        .unwrap();
    let get_01_transact = aws_sdk_dynamodb::types::TransactGetItem::builder()
        .get(get_01_action)
        .build();
    let get_result = client
        .transact_get_items()
        .transact_items(get_01_transact)
        .send()
        .await;
    match get_result {
        Ok(val) => {
            if let Some(items) = val.responses {
                for item in items {
                    println!("{:?}", item.item);
                }
            }
        }
        Err(err) => match err.into_service_error() {
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::InternalServerError(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::InvalidEndpointException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::ProvisionedThroughputExceededException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::RequestLimitExceeded(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::ResourceNotFoundException(_) => todo!(),
            aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsError::TransactionCanceledException(_) => todo!(),
            err if err.code() == Some("Unhandled") => todo!(),
            _err => todo!(),
        },
    };
    Ok(())
}
