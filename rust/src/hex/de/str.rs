use std::{fmt, marker::PhantomData};

use serde::{Deserializer, de::Visitor};

use crate::hex::Hex;

pub trait HexDeserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de> HexDeserialize<'de> for Hex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HexVisitor;

        impl<'de> Visitor<'de> for HexVisitor {
            type Value = Hex;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hexadecimal string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Hex::from(value))
            }
        }

        deserializer.deserialize_str(HexVisitor)
    }
}

impl<'de, T> HexDeserialize<'de> for Option<T>
where
    T: HexDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for OptionVisitor<T>
        where
            T: HexDeserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an optional hexadecimal string")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let hex = T::deserialize(deserializer)?;
                Ok(Some(hex))
            }
        }

        deserializer.deserialize_option(OptionVisitor {
            marker: PhantomData,
        })
    }
}

impl<'de> HexDeserialize<'de> for Vec<Hex> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecHexVisitor;

        impl<'de> Visitor<'de> for VecHexVisitor {
            type Value = Vec<Hex>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of hexadecimal strings")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut items = Vec::new();

                while let Some(value) = seq.next_element::<String>()? {
                    items.push(Hex::from(value));
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(VecHexVisitor)
    }
}

impl<'de> HexDeserialize<'de> for (Hex, Hex) {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TupleHexVisitor;

        impl<'de> Visitor<'de> for TupleHexVisitor {
            type Value = (Hex, Hex);

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a tuple of hexadecimal strings")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let first = seq
                    .next_element::<String>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let second = seq
                    .next_element::<String>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok((Hex::from(first), Hex::from(second)))
            }
        }

        deserializer.deserialize_tuple(2, TupleHexVisitor)
    }
}
