use mingling::{
    ShellContext, Suggest,
    macros::{help, suggest},
    prelude::*,
};

use crate::commands::EntryGreet;

#[help]
pub fn help_greet(_p: EntryGreet) {
    r_println!(
        "{}",
        r"
This is a sample command used to greet someone.
Usage: greet <NAME>

Example: greet Alice
        "
        .trim()
    );
}

#[completion(EntryGreet)]
pub fn complete_greet(_ctx: &ShellContext) -> Suggest {
    suggest! {
        "Alice",
        "Bob",
        "Peter"
    }
}
