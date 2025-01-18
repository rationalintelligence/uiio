use anyhow::Result;
use uiio_element::Progress;

fn main() -> Result<()> {
    let fqn = "my.stream.progress_1".parse()?;
    let mut progress_1 = Progress::new(fqn, 100);
    let fqn = "my.stream.progress_2".parse()?;
    let mut progress_2 = Progress::new(fqn, 100);
    for x in 0..=100 {
        progress_1.set_value(x);
        progress_2.set_value(x);
    }
    Ok(())
}
