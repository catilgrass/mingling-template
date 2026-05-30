use mingling::prelude::*;

use crate::{Next, renderers::ResultGreetSomeone};

// Greet Dispatcher
dispatcher!("greet", CMDGreet => EntryGreet);

#[chain]
pub fn handle_greet(prev: EntryGreet) -> Next {
    let (repeat, name) = prev
        .pick(["--repeat", "-r"])
        .pick_or::<String>((), "World")
        .unpack();
    ResultGreetSomeone { name, repeat }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mingling::{ChainProcess, assert_chain_result, assert_member_id, macros::entry};

    #[test]
    fn test_handle_greet() {
        let result = handle_greet(entry!("Alice", "--repeat", "6")).into();
        assert_chain_result!(&result);
        assert_member_id!(&result, crate::ThisProgram::ResultGreetSomeone);
        let ChainProcess::Ok((any, _)) = result else {
            panic!("Result of `handle_greet` is not Ok")
        };
        let result_greet_someone = any
            .downcast::<ResultGreetSomeone>()
            .expect("Failed to downcast to ResultGreetSomeone");
        assert_eq!(result_greet_someone.name, "Alice");
        assert_eq!(result_greet_someone.repeat, 6);
    }
}
