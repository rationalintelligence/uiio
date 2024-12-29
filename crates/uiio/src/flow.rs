use serde::Serialize;
use std::any::type_name;

pub trait OutFlow: Default + Serialize {
    // TODO: No serialize needed
    type Value: Clone + Serialize;

    fn class(&self) -> &str {
        type_name::<Self>()
    }
}
