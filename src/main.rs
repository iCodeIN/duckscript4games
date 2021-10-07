use duckscript::{
    runner,
    types::{
        command::{Command, CommandResult},
        error::ScriptError,
        runtime::Context,
    },
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct GameState {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct MoveRightCommand {
    state: Rc<RefCell<GameState>>,
}

fn main() -> Result<(), ScriptError> {
    let state = Rc::new(RefCell::new(GameState { x: 0.0, y: 0.0 }));

    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    context.commands.set(Box::new(MoveRightCommand {
        state: state.clone(),
    }))?;

    runner::run_script("moveright", context)?;

    dbg!(state);

    Ok(())
}

impl Command for MoveRightCommand {
    fn name(&self) -> String {
        "moveright".to_string()
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        let mut state = self.state.borrow_mut();
        state.x += 10.;
        CommandResult::Continue(Some(state.x.to_string()))
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
