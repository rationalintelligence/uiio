use crate::flow::OutFlow;
use crate::fqn::Fqn;
use crate::tracer::Tracer;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct ProgressFlow;

impl OutFlow for ProgressFlow {
    type Value = ProgressValue;
}

#[derive(Serialize, Clone)]
pub struct ProgressValue {
    progress: u32,
}

pub struct Progress {
    tracer: Tracer<ProgressFlow>,
    current: u64,
    total: u64,
    value: ProgressValue,
}

impl Progress {
    pub fn new(fqn: Fqn, total: u64) -> Self {
        let value = ProgressValue { progress: 0 };
        let tracer = Tracer::new(fqn, &value);
        Self {
            tracer,
            current: 0,
            total,
            value,
        }
    }

    pub fn set_value(&mut self, value: u64) {
        self.current = value;
        let prev_value = self.value.progress;
        let new_value = (self.current * 100 / self.total) as u32;
        if prev_value != new_value {
            self.value.progress = new_value;
            self.tracer.trace(&self.value);
        }
    }
}
