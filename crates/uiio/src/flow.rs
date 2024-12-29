use crate::fqn::Fqn;
use crate::tracer::{Trace, Tracer};
use serde::Serialize;

pub struct ProgressTrace;

impl Trace for ProgressTrace {
    type Value = ProgressValue;
}

#[derive(Serialize)]
pub struct ProgressValue {
    progress: u32,
}

pub struct Progress {
    tracer: Tracer<ProgressTrace>,
    current: u64,
    total: u64,
    value: ProgressValue,
}

impl Progress {
    pub fn new(fqn: Fqn, total: u64) -> Self {
        Self {
            tracer: Tracer::new(fqn),
            current: 0,
            total,
            value: ProgressValue { progress: 0 },
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
