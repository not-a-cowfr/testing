#![allow(non_camel_case_types)]

use std::fmt;

use hex::{decode, encode};

mod de;
mod ser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hex(Vec<u8>);

impl Hex {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn to_alphanumeric(&self) -> String {
        encode(&self.0)
    }

    pub fn to_numeric(&self) -> u64 {
        let hex_str = self.to_alphanumeric();
        u64::from_str_radix(&hex_str, 16).expect("This shouldn't fail")
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_alphanumeric())
    }
}

impl From<&str> for Hex {
    fn from(data: &str) -> Hex {
        Hex(decode(data).unwrap())
    }
}

impl From<String> for Hex {
    fn from(data: String) -> Hex {
        Hex(decode(&data).unwrap())
    }
}

impl From<u64> for Hex {
    fn from(data: u64) -> Hex {
        let hex_string = format!("{:x}", data);

        // fixes panic when trying to convert "f" or similar to a [`Hex`]
        let padded = if hex_string.len() % 2 != 0 {
            format!("0{}", hex_string)
        } else {
            hex_string
        };

        Hex(decode(padded).unwrap())
    }
}

impl From<Vec<u8>> for Hex {
    fn from(data: Vec<u8>) -> Hex {
        Hex(data)
    }
}

/// serializes/deserializes [`Hex`] to/from the encoded, string format
pub mod as_str {
    use serde::{self, Deserializer, Serializer};

    use crate::hex::{de::str::HexDeserialize, ser::str::HexSerialize};

    pub fn serialize<'a, T, S>(hex: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: HexSerialize,
        S: Serializer,
    {
        HexSerialize::serialize(hex, serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: HexDeserialize<'de>,
        D: Deserializer<'de>,
    {
        HexDeserialize::deserialize(deserializer)
    }
}

/// serializes/deserialzies [`Hex`] to/from the decoded, numerical format
pub mod as_num {
    use serde::{self, Deserializer, Serializer};

    use crate::hex::{de::num::HexDeserialize, ser::num::HexSerialize};

    pub fn serialize<'a, T, S>(hex: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: HexSerialize,
        S: Serializer,
    {
        HexSerialize::serialize(hex, serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: HexDeserialize<'de>,
        D: Deserializer<'de>,
    {
        HexDeserialize::deserialize(deserializer)
    }
}
