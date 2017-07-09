#[macro_use]
extern crate nom;

pub mod reader;
pub mod parser;
pub mod error_handler;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
