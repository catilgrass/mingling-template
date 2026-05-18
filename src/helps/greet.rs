use mingling::macros::{help, suggest};
use mingling::prelude::*;
use mingling::{ShellContext, Suggest};

use crate::commands::EntryGreet;

#[help]
pub fn help_greet(_prev: EntryGreet) {
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
pub fn comp_greet(_ctx: &ShellContext) -> Suggest {
    suggest! {
        "Alice",
        "Bob",
        "Peter"
    }
}
