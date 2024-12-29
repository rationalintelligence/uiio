use crate::fqn::Fqn;
use crate::protocol::{Event, Record, UiOutWrite};
use serde::Serialize;
use std::any::type_name;

pub trait OutFlow: Default + Serialize {
    // TODO: No serialize needed
    type Value: Clone + Serialize;

    fn class(&self) -> &str {
        type_name::<Self>()
    }
}

pub struct Tracer<F: OutFlow> {
    fqn: String,
    flow: F,
}

impl<F> Tracer<F>
where
    F: OutFlow,
{
    pub fn new(fqn: Fqn, value: &F::Value) -> Self {
        let this = Self {
            fqn: fqn.to_string(),
            flow: F::default(),
        };
        this.create();
        this.trace(value);
        this
    }

    fn create(&self) {
        let class = self.flow.class().to_string();
        let event = Event::Create(class);
        self.event(event);
    }

    fn destroy(&self) {
        let event = Event::Destroy;
        self.event(event);
    }

    fn event(&self, event: Event<F>) {
        let record = Record {
            fqn: self.fqn.clone(),
            event,
        };
        if let Ok(dump) = serde_json::to_string(&record) {
            println!("{}", dump);
        }
    }

    pub fn trace(&self, value: &F::Value) {
        let event = Event::Value(value.clone());
        self.event(event);
    }
}

impl<F> Drop for Tracer<F>
where
    F: OutFlow,
{
    fn drop(&mut self) {
        self.destroy();
    }
}
