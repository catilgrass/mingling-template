mod commands;
use commands::*;

mod helps;
use helps::*;

mod renderers;
use renderers::*;

use crate::commands::greet::CommandGreet;

use mingling::setup::BasicProgramSetup;

fn main() {
    let mut program = ThisProgram::new();

    // Plugins
    program.with_setup(BasicProgramSetup);
    program.with_dispatcher(CompletionDispatcher);

    // Dispatchers
    program.with_dispatcher(CommandGreet);

    program.exec();
}

mingling::macros::gen_program!();
