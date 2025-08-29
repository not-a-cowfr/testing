use serde::{Serializer, ser::SerializeSeq};

use crate::hex::Hex;

pub trait HexSerialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl HexSerialize for Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.to_numeric())
    }
}

impl<T> HexSerialize for Option<T>
where
    T: HexSerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(hex) = self {
            HexSerialize::serialize(hex, serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl HexSerialize for Vec<Hex> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for item in self.iter() {
            seq.serialize_element(&item.to_numeric())?;
        }

        seq.end()
    }
}

impl HexSerialize for (Hex, Hex) {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0.to_numeric())?;
        seq.serialize_element(&self.1.to_numeric())?;
        seq.end()
    }
}

impl<T> HexSerialize for &T
where
    T: ?Sized + HexSerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}
