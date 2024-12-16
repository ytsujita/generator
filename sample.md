カスタムデシリアライズの例

use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

#[derive(Debug)]
struct MyStruct {
    pk: String,
}

// カスタムデシリアライズを実装
impl<'de> Deserialize<'de> for MyStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MyStructVisitor)
    }
}

struct MyStructVisitor;

impl<'de> Visitor<'de> for MyStructVisitor {
    type Value = MyStruct;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with UserId and Fizz fields")
    }

    fn visit_map<V>(self, mut map: V) -> Result<MyStruct, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut user_id = None;
        let mut fizz = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "UserId" => {
                    if user_id.is_some() {
                        return Err(de::Error::duplicate_field("UserId"));
                    }
                    user_id = Some(map.next_value()?);
                }
                "Fizz" => {
                    if fizz.is_some() {
                        return Err(de::Error::duplicate_field("Fizz"));
                    }
                    fizz = Some(map.next_value()?);
                }
                _ => {
                    let _ = map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        let user_id: String = user_id.ok_or_else(|| de::Error::missing_field("UserId"))?;
        let fizz: String = fizz.ok_or_else(|| de::Error::missing_field("Fizz"))?;

        Ok(MyStruct {
            pk: format!("{}#{}", user_id, fizz),
        })
    }
}

fn main() {
    let json_data = r#"{"UserId": "12345", "Fizz": "buzz"}"#;
    let my_struct: MyStruct = serde_json::from_str(json_data).unwrap();
    println!("{:?}", my_struct);
}
