use askama::Template;

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
#[template(path = "dynamodb/accessor.py", escape = "none")]
pub(super) struct DynamoDbAccessorTemplate<'a> {
    pub(super) dynamodb: &'a str,
}

pub(crate) fn generate_dynamodb_accessor() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
