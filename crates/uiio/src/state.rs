use crate::names::{FlowId, Fqn};
use crate::protocol::{EventDe, RecordDe};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

pub struct Element {
    pub fqn: Fqn,
    pub class: String,
    pub value: Option<Value>,
}

#[derive(Default)]
pub struct Level {
    pub levels: BTreeMap<String, Level>,
    pub flows: HashSet<FlowId>,
}

impl Level {
    pub fn discover(&mut self, fqn: &Fqn) -> &mut Level {
        let mut level = self;
        for segment in fqn.iter() {
            level = level.levels.entry(segment.into()).or_default();
        }
        level
    }
}

pub struct UiioState {
    events: Vec<RecordDe>,
    elements: BTreeMap<FlowId, Element>,
    tree: Level,
}

impl UiioState {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            elements: BTreeMap::new(),
            tree: Level::default(),
        }
    }

    pub fn tree(&self) -> &Level {
        &self.tree
    }

    pub fn add_event(&mut self, record: RecordDe) {
        let id = record.id;
        match &record.event {
            EventDe::Create { fqn, class } => {
                let level = self.tree.discover(&fqn);
                level.flows.insert(id);
                let element = Element {
                    fqn: fqn.clone(),
                    class: class.clone(),
                    value: None,
                };
                self.elements.insert(id, element);
            }
            EventDe::Value(value) => {
                if let Some(element) = self.elements.get_mut(&id) {
                    element.value = Some(value.clone());
                }
            }
            EventDe::Destroy => {
                if let Some(element) = self.elements.remove(&id) {
                    let fqn = element.fqn;
                    let level = self.tree.discover(&fqn);
                    level.flows.remove(&id);
                }
            }
        }
        self.events.push(record);
    }
}
