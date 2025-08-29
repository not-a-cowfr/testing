#![cfg(test)]

use crate::*;

#[test]
fn basic_serialization() {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Thing {
        #[serde(with = "hex::as_str")]
        foo: Hex,
        #[serde(with = "hex::as_num")]
        bar: Hex,
    }

    let thing = Thing {
        foo: "123abc".into(),
        bar: 16777215.into(),
    };

    let json = serde_json::to_string(&thing).unwrap();

    let thing2: Thing = serde_json::from_str(&json).unwrap();

    assert_eq!(thing, thing2);
}
