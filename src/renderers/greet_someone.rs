use crate::ThisProgram;
use mingling::{
    Groupped,
    macros::{r_println, renderer},
};

#[derive(Debug, Groupped)]
pub struct ResultGreetSomeone {
    pub name: String,
}

#[renderer]
pub fn render_greet_someone(prev: ResultGreetSomeone) {
    r_println!("Hello, {}!", prev.name)
}
