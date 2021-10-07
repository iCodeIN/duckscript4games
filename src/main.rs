use duckscript::{
    runner,
    types::{
        command::{Command, CommandResult},
        error::ScriptError,
        runtime::Context,
    },
};

#[derive(Debug)]
struct GameState {
    x: f32,
    y: f32,
}

struct MoveRightCommand<'a> {
    state: &'a mut GameState,
}

fn main() {
    let state = GameState { x: 0.0, y: 0.0 };

    execute_script("moveright", &mut state).unwrap();

    dbg!(state);
}

fn execute_script(script: &str, state: &mut GameState) -> Result<Context, ScriptError> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    context.commands.set(Box::new(MoveRightCommand { state }))?;

    runner::run_script(script, context)
}

impl Command for MoveRightCommand<'_> {
    fn name(&self) -> String {
        "moveright".to_string()
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        self.state.x += 10.;
        CommandResult::Continue(Some(self.state.x.to_string()))
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
