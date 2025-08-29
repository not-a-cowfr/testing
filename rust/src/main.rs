use serde::{Deserialize, Serialize};

use crate::hex::Hex;

mod hex;
mod local_storage;
mod tests;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Thing {
    #[serde(with = "hex::as_str")]
    foo: Hex,
    #[serde(with = "hex::as_str")]
    bar: Option<Hex>,
    #[serde(with = "hex::as_str")]
    barbar: Option<Option<Hex>>,
    #[serde(with = "hex::as_str")]
    buzz: Vec<Hex>,
    #[serde(with = "hex::as_str")]
    buz: (Hex, Hex),
}

fn main() {
    let thing = Thing {
        foo: "123abc".into(),
        bar: Some(16777215.into()),
        barbar: Some(Some("123456".into())),
        buzz: vec![vec![1, 10, 100].into()],
        buz: ("ffffff".into(), "000000".into()),
    };

    let json = serde_json::to_string(&thing).unwrap();

    println!("{:?}", json);

    let thing2: Thing = serde_json::from_str(&json).unwrap();

    println!("{:?}", thing2);

    assert_eq!(thing, thing2);
}
