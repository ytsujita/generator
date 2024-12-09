pub(crate) struct DynamoDbTableConfig {
    hash_key: String,
    range_key: Option<String>,
    ttl_attribute_name: String,
    attributes: Vec<DynamoDbTableAttributeConfig>,
}

pub(crate) struct DynamoDbTableGlobalSecondaryIndexConfig {
    name: String,
    hash_key: String,
    range_key: Option<String>,
    projection_type: DynamoDbProjectionType,
}

pub(crate) struct DynamoDbTableAttributeConfig {
    name: String,
    attr_type: DynamoDbTableAttributeType,
    attributes: Vec<DynamoDbTableKeyType>,
}

pub(crate) enum DynamoDbTableKeyType {
    String,
    Number,
    Binary,
}

pub(crate) enum DynamoDbTableAttributeType {
    String,
    Number,
    Binary,
    Boolean,
    Null,
    List,
    Map,
    StringSet,
    NumberSet,
    BinarySet,
}

pub(crate) enum DynamoDbProjectionType {
    All,
    Include,
    KeysOnly,
}
