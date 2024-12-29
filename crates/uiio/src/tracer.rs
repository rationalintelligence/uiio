use crate::flow::OutFlow;
use crate::fqn::Fqn;
use crate::protocol::{Event, Record};
use std::marker::PhantomData;

pub struct Tracer<OUT: OutFlow> {
    fqn: String,
    _out: PhantomData<OUT>,
}

impl<OUT> Tracer<OUT>
where
    OUT: OutFlow,
{
    pub fn new(fqn: Fqn, value: &OUT) -> Self {
        let this = Self {
            fqn: fqn.to_string(),
            _out: PhantomData,
        };
        this.create();
        this.trace(value);
        this
    }

    fn create(&self) {
        let class = OUT::class();
        let event = Event::Create(class);
        self.event(event);
    }

    fn destroy(&self) {
        let event = Event::Destroy;
        self.event(event);
    }

    fn event(&self, event: Event<OUT>) {
        let record = Record {
            fqn: self.fqn.clone(),
            event,
        };
        if let Ok(dump) = serde_json::to_string(&record) {
            println!("{}", dump);
        }
    }

    pub fn trace(&self, value: &OUT) {
        let event = Event::Value(value);
        self.event(event);
    }
}

impl<OUT> Drop for Tracer<OUT>
where
    OUT: OutFlow,
{
    fn drop(&mut self) {
        self.destroy();
    }
}
