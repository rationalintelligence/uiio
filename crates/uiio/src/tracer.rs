use crate::fqn::Fqn;
use crate::protocol::UiOut;
use serde::Serialize;
use std::marker::PhantomData;

pub trait Trace {
    type Value: Serialize;
}

pub struct Tracer<T> {
    fqn: Fqn,
    _value: PhantomData<T>,
}

impl<T> Tracer<T>
where
    T: Trace,
{
    pub fn new(fqn: Fqn) -> Self {
        Self {
            fqn,
            _value: PhantomData,
        }
    }

    pub fn trace(&self, value: &T::Value) {
        let out = UiOut {
            fqn: self.fqn.as_ref(),
            value,
        };
        // TODO: Use `to_writer` instead
        if let Ok(dump) = serde_json::to_string(&out) {
            println!("{}", dump);
        }
    }
}
