use derive_more::{From, Into};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, From, Into, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackedData(pub Vec<u8>);

pub trait DataFraction
where
    Self: DeserializeOwned + Serialize,
    Self: Clone + Sync + Send + 'static,
{
}

impl<T> DataFraction for T
where
    T: DeserializeOwned + Serialize,
    T: Clone + Sync + Send + 'static,
{
}

pub trait Flow: DataFraction {
    type Action: DataFraction;
    type Event: DataFraction;

    fn class() -> &'static str {
        type_name::<Self>()
    }

    fn apply(&mut self, event: Self::Event);
}

pub trait OutFlow: Default + Serialize {
    // TODO: No serialize needed
    type Value: Clone + Serialize;

    fn class(&self) -> &str {
        type_name::<Self>()
    }
}
