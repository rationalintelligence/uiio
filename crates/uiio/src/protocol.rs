use serde::Serialize;

#[derive(Serialize)]
pub struct UiOut<'a, V> {
    pub fqn: &'a str,
    pub value: &'a V,
}
