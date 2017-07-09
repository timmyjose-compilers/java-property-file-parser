extern crate jpropparser;

use jpropparser::*;
use error_handler as eh;

/// The Main Man!
fn main() {
    match reader::get_args() {
        Err(e) => {
            if e.kind() == &eh::PropErrorKind::InsufficientArgs {
                eh::display_usage();
                eh::exit_error(e);
            } else {
                error_handler::exit_error(e);
            }
        }

        Ok(args) => {
            for arg in args.iter() {
                match reader::get_bytes(arg) {
                    Err(e) => error_handler::exit_error(e),

                    Ok(bytes) => {
                        let parsed = parser::process(&bytes);

                        for (key, val) in parsed.unwrap().iter() {
                            println!("{} = {}", key, val);
                        }
                        println!();
                    }
                }
            }
        }
    }
}
