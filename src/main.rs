mod commands;
use commands::*;

mod helps;
use helps::*;

mod renderers;
use renderers::*;

use crate::commands::greet::CommandGreet;

use mingling::{
    macros::{r_println, renderer},
    setup::BasicProgramSetup,
};

fn main() {
    let mut program = ThisProgram::new();

    // Plugins
    program.with_setup(BasicProgramSetup);
    program.with_dispatcher(CompletionDispatcher);

    // Dispatchers
    program.with_dispatcher(CommandGreet);

    program.exec();
}

#[renderer]
fn handle_fallback_dispatcher_not_found(prev: DispatcherNotFound) {
    r_println!("Error: cannot match \"{}\" to any command", prev.join(" "));
}

#[renderer]
fn handle_fallback_renderer_not_found(prev: RendererNotFound) {
    let type_name = prev.inner;
    r_println!("Error: renderer not found for \"{}\"", type_name);
}

mingling::macros::gen_program!();
