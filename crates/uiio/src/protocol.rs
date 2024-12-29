use crate::tracer::OutFlow;
use serde::{Deserialize, Serialize};

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
