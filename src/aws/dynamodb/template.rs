use askama::Template;

use super::config::{DynamoDBTableConfig, DynamoDBTableKey};

pub(crate) enum DynamoDbAccessPattern {
    GetItem,
    PutItem,
    DeleteItem,
    Query,
    Scan,
    TransactWriteItems,
    TransactGetItems,
}

#[derive(Template)]
#[template(path = "aws/dynamodb/accessor.py", escape = "none")]
pub(super) struct DynamoDbAccessorTemplate<'a> {
    pub(super) dynamodb: &'a str,
}

pub(crate) fn generate_dynamodb_accessor() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
