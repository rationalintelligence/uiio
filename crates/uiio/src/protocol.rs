use crate::flow::OutFlow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Record<T: OutFlow> {
    pub fqn: String,
    pub event: Event<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Event<T: OutFlow> {
    Create(String),
    Value(T::Value),
    Destroy,
}

#[derive(Serialize)]
pub struct UiOutWrite<'a, V> {
    pub fqn: &'a str,
    pub value: &'a V,
}

#[derive(Deserialize)]
pub struct UiOutRead<V> {
    pub fqn: String,
    pub value: V,
}

#[derive(Deserialize)]
pub struct RecordRead {
    pub fqn: String,
    pub event: EventRead,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventRead {
    Create(String),
    Value(Value),
    Destroy,
}
