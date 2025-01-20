use crate::names::FlowId;
use std::collections::BTreeMap;

pub struct UiioState {
    elements: BTreeMap<FlowId, ()>,
}

impl UiioState {
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
        }
    }
}
