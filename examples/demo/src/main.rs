use anyhow::Result;
use uiio_element::{Progress, State};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq)]
enum AppState {
    Init,
    Progress,
    Done,
}

fn main() -> Result<()> {
    let fqn = "my.app.state".parse()?;
    let mut state = State::new(fqn, AppState::Init);

    let fqn = "my.stream.progress_1".parse()?;
    let mut progress_1 = Progress::new(fqn, 100);

    let fqn = "my.stream.progress_2".parse()?;
    let mut progress_2 = Progress::new(fqn, 100);

    state.set_state(AppState::Progress);
    for x in 0..=100 {
        progress_1.set_value(x);
        progress_2.set_value(x);
    }

    state.set_state(AppState::Done);
    Ok(())
}
