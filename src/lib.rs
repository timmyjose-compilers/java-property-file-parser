#[macro_use]
extern crate nom;

pub mod reader;
pub mod parser;
pub mod error_handler;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::parser::*;

    #[test]
    fn it_works() {
        let string = b"name = Goofy\n #a charlatan!\n  address = unknown\n";

        let mut check = HashMap::new();

        check.insert("name", "Goofy");
        check.insert("address", "unknown");

        assert_eq!(process(string).unwrap(), check);
    }

    #[test]
    fn this_also_works() {
        let string = b"hello     =     world \n";

        let mut check = HashMap::new();
        check.insert("hello", "world ");
        
        assert_eq!(process(string).unwrap(), check);
    }
}
