use crate::flow::EventFlow;
use crate::names::{FlowId, Fqn};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Record<'a, T: EventFlow> {
    pub id: FlowId,
    pub event: Event<'a, T>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Event<'a, T: EventFlow> {
    Create { fqn: &'a Fqn, class: &'a str },
    Value(&'a T),
    Destroy,
}

#[derive(Serialize)]
pub struct RecordDe<T: EventFlow> {
    pub id: FlowId,
    pub event: EventDe<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventDe<T: EventFlow> {
    Create { fqn: Fqn, class: String },
    Value(T),
    Destroy,
}
