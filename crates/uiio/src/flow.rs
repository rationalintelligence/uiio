use anyhow::{Error, Result};
use derive_more::{From, Into};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::any::type_name;

pub trait DataFraction
where
    Self: DeserializeOwned + Serialize,
    Self: Clone + Sync + Send + 'static,
{
    type Block;

    fn pack(&self) -> Result<Self::Block>;

    fn unpack(data: Self::Block) -> Result<Self>;
}

impl<T> DataFraction for T
where
    T: DeserializeOwned + Serialize,
    T: Clone + Sync + Send + 'static,
{
    type Block = Value;

    fn pack(&self) -> Result<Self::Block> {
        serde_json::to_value(self).map_err(Error::from)
    }

    fn unpack(data: Self::Block) -> Result<Self> {
        serde_json::from_value(data).map_err(Error::from)
    }
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
