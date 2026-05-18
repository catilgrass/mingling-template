use mingling::prelude::*;

use crate::renderers::ResultGreetSomeone;

// Greet Dispatcher
dispatcher!("greet", CommandGreet => EntryGreet);

#[chain]
pub fn state_parse_greet(prev: EntryGreet) -> Next {
    let name = prev.pick_or::<String>((), "World").unpack();
    ResultGreetSomeone { name }
}
