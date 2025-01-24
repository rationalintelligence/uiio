#[macro_export]
macro_rules! out {
    ($fqdn:expr, $value:expr) => {{
        println!("{}={}", $fqdn, $value);
    }};
}
