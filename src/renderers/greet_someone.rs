use mingling::Groupped;
use mingling::prelude::*;

#[derive(Debug, Groupped)]
pub struct ResultGreetSomeone {
    pub name: String,
}

#[renderer]
pub fn render_greet_someone(prev: ResultGreetSomeone) {
    r_println!("Hello, {}!", prev.name)
}
