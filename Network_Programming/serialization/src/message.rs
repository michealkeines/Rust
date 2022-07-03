extern crate serde;
extern crate serde_json;
extern crate serde_test;
use std::time::Instant;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use std::time::Duration;
use std::ops::Sub;

const FIELDS: &'static [&'static str] = &["value", "timestamp"];

#[derive(Debug, PartialEq)]
pub struct Message {
    pub value: String,
    pub timestamp: Instant
}

impl Serialize for Message {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Message", 2)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("timestamp", &self.timestamp.elapsed())?;
        state.end()
    }
}

/*
    struct kuge {
        port: u8,
        healthz_port: u8,
        max_pods: u8
    }
*/

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field { Value, Timestamp }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
                struct FieldVistor;

                impl<'de> Visitor<'de> for FieldVistor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value` or `timestamp`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E> where E: de::Error {
                        match value {
                            "value" => Ok(Field::Value),
                            "timestamp" => Ok(Field::Timestamp),
                            _ => Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVistor)
            }
        }

        struct MessageVisitor;

        impl<'de> Visitor<'de> for MessageVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Message")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Message, V::Error> {
                let mut value = None;
                let mut timestamp = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Timestamp => {
                            // if value.is_some() {
                            //     return Err(de::Error::duplicate_field("timestamp"));
                            // }
                            let temp: Duration = map.next_value()?;
                            let now = Instant::now();
                            timestamp = Some(now.sub(temp));
                        }

                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;

                Ok(Message {
                    value: value,
                    timestamp: timestamp
                })
            }
        }
        deserializer.deserialize_struct("Message", FIELDS, MessageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_de_tokens};

    #[test]
    fn test_ser_de() {
        let message = Message {
            value : String::from("This is my Message"),
            timestamp: Instant::now()
        };

        assert_de_tokens(&message, &[
            Token::Struct {
                name: "Message",
                len: 2
            },
            Token::Str("value"),
            Token::String("This is my Message"),
            Token::Str("timestamp"),
            Token::Struct {
                name: "Duration",
                len: 2
            },
            Token::StructEnd

        ]);
    }
    
}