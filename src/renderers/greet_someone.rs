use mingling::{Groupped, prelude::*};

#[derive(Debug, Groupped)]
pub struct ResultGreetSomeone {
    pub name: String,
    pub repeat: u8,
}

#[renderer]
pub fn render_greet_someone(result: ResultGreetSomeone) -> String {
    let repeat = result.repeat as usize;
    r_println!("Hello, {}!", result.name.repeat(repeat))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_greet_someone() {
        let result = render_greet_someone(ResultGreetSomeone {
            name: "Bob".to_string(),
            repeat: 3,
        });
        assert_eq!(result, "Hello, BobBobBob!\n")
    }
}
