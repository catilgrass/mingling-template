mod commands;
use commands::*;

mod helps;
use helps::*;

mod renderers;
use renderers::*;

use mingling::prelude::*;
use mingling::setup::{BasicProgramSetup, ExitCodeSetup};

fn main() {
    let mut program = ThisProgram::new();

    // Plugins
    program.with_setup(BasicProgramSetup);
    program.with_setup(ExitCodeSetup::default());

    // Completion Dispatcher
    program.with_dispatcher(CMDCompletion);

    // Dispatchers
    program.with_dispatcher(CMDGreet);

    program.exec();
}

#[renderer]
fn handle_fallback_dispatcher_not_found(prev: ErrorDispatcherNotFound) {
    r_println!("Error: cannot match \"{}\" to any command", prev.join(" "));
}

#[renderer]
fn handle_fallback_renderer_not_found(prev: ErrorRendererNotFound) {
    let type_name = prev.inner;
    r_println!("Error: renderer not found for \"{}\"", type_name);
}

gen_program!();
