use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum UseCaseDefinition {
    Query(QeuryUseCaseDefinition),
    Command(CommandUseCaseDefinition),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct QeuryUseCaseDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) query_parameters: Option<Vec<Scheme>>,
    pub(crate) request_body: Option<Vec<Scheme>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CommandUseCaseDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) request_body: Option<Vec<Scheme>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Scheme {
    String {
        format: SchemeStringFormat,
        example: Option<String>,
    },
    Number {
        format: SchemeNumberFormat,
        example: Option<f64>,
    },
    Integer {
        format: SchemeIntegerFormat,
        example: Option<i64>,
    },
    Boolean {
        example: Option<bool>,
    },
    Array {
        scheme_type: Box<SchemeType>,
    },
    Object {
        properties: (String, Box<SchemeType>),
    },
}

#[derive(Serialize, Deserialize)]
pub(crate) enum SchemeType {}

#[derive(Serialize, Deserialize)]
pub(crate) enum SchemeStringFormat {
    Date,     // 日付 (例: 2024-01-20)
    DateTime, // 日時 (例: 2024-01-20T12:00:00Z)
    Password, // パスワード
    Byte,     // base64エンコードされた文字列
    Binary,   // バイナリデータ
    Email,    // メールアドレス
    Uuid,     // UUID
    Uri,      // URI
    Hostname, // ホスト名
    Ipv4,     // IPv4アドレス
    Ipv6,     // IPv6アドレス
}

#[derive(Serialize, Deserialize)]
pub(crate) enum SchemeNumberFormat {
    Float,  // 32ビット浮動小数点
    Double, // 64ビット浮動小数点
}

#[derive(Serialize, Deserialize)]
pub(crate) enum SchemeIntegerFormat {
    Int32, // 32ビット整数
    Int64, // 64ビット整数
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SchemeProperties {}

#[derive(Serialize, Deserialize)]
pub(crate) struct DynamoDBTableConfig {
    pub(crate) hash_key: DynamoDBTableKey,
    pub(crate) range_key: Option<DynamoDBTableKey>,
    pub(crate) ttl_attribute_name: String,
    pub(crate) attributes: Vec<DynamoDBTableAttributeConfig>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DynamoDBTableKey {
    pub(crate) key_name: String,
    pub(crate) key_type: DynamoDBTableKeyType,
    pub(crate) key_format: String,     // {sample}#{foobar}
    pub(crate) value_key: Vec<String>, // ["sample", "foobar"]
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DynamoDBTableGlobalSecondaryIndexConfig {
    pub(crate) name: String,
    pub(crate) hash_key: String,
    pub(crate) range_key: Option<String>,
    pub(crate) projection_type: DynamoDBProjectionType,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DynamoDBTableAttributeConfig {
    pub(crate) attr_name: String,
    pub(crate) attr_type: DynamoDBTableAttributeType,
    pub(crate) attr_format: String,    // {sample}#{foobar}
    pub(crate) value_key: Vec<String>, // ["sample", "foobar"]
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DynamoDBTableKeyType {
    String,
    Number,
    Binary,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DynamoDBTableAttributeType {
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

#[derive(Serialize, Deserialize)]
pub(crate) enum DynamoDBProjectionType {
    All,
    Include,
    KeysOnly,
}
