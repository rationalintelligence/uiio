use anyhow::Result;
use uiio_element::{Progress, State};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq)]
enum AppState {
    Init,
    Progress,
    Done,
}

#[derive(Serialize, PartialEq, Eq)]
enum AppLoop {
    Begin,
    End,
}

const TOTAL: u64 = 100_000;

fn main() -> Result<()> {
    let fqn = "my.app.state".parse()?;
    let mut state = State::new(fqn, AppState::Init);

    let fqn = "my.stream.progress_1".parse()?;
    let mut progress_1 = Progress::new(fqn, TOTAL);

    let fqn = "my.stream.progress_2".parse()?;
    let mut progress_2 = Progress::new(fqn, TOTAL);

    state.set_state(AppState::Progress);

    let fqn = "my.app.loop".parse()?;
    let mut app_loop = State::new(fqn, AppLoop::Begin);
    app_loop.set_state(AppLoop::Begin);
    for x in 0..=TOTAL {
        progress_1.set_value(x);
        progress_2.set_value(x);
    }
    app_loop.set_state(AppLoop::End);

    state.set_state(AppState::Done);
    Ok(())
}
