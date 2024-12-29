use serde::Serialize;

pub trait OutFlow: Serialize {
    fn class() -> &'static str;
}
