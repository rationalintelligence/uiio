use crate::names::Fqn;

#[macro_export]
macro_rules! name {
    ($($item:expr),* $(,)?) => {{
        let components = vec![$($item.to_string()),*];
        $crate::names::Pqn { components }
    }};
}

#[macro_export]
macro_rules! partial {
    ($text:expr) => {{
        let components = const_str::split!($text, '.');
        $crate::names::Pqn::from_iter(components)
    }};
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pqn {
    components: Vec<String>,
}

impl Pqn {
    pub fn from_iter<'a>(components: impl IntoIterator<Item = &'a str>) -> Self {
        let components = components.into_iter().map(String::from).collect();
        Self { components }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ str> {
        self.components.iter().map(String::as_ref)
    }

    pub fn into_full(self) -> Fqn {
        Fqn::from_iter(self.iter())
    }
}

impl From<Pqn> for Fqn {
    fn from(pqn: Pqn) -> Self {
        pqn.into_full()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_name_macro() -> Result<()> {
        let expected = "one.two".parse()?;
        let name = Fqn::from(name!("one", "two"));
        assert_eq!(name, expected);
        Ok(())
    }

    #[test]
    fn test_partial_macro() -> Result<()> {
        let expected = "one.two".parse()?;
        let name = Fqn::from(partial!("one.two"));
        assert_eq!(name, expected);
        Ok(())
    }
}
