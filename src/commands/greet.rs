use mingling::{
    macros::{chain, dispatcher},
    parser::AsPicker,
};

use crate::{ThisProgram, renderers::greet_someone::ResultGreetSomeone};

// Greet Dispatcher
dispatcher!("greet", CommandGreet => EntryGreet);

#[chain]
pub fn state_parse_greet(prev: EntryGreet) -> NextProcess {
    let name = prev.pick_or::<String>((), "World").unpack();
    ResultGreetSomeone { name }
}
