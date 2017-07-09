use std::io::{Read, BufReader};
use std::fs::File;

use error_handler as eh;

/// ensure that we have at least one property
/// file specified on the command line
pub fn get_args() -> eh::PropResult<Vec<String>> {
    let args = ::std::env::args()
        .skip(1)
        .collect::<Vec<_>>();

    if args.len() == 0 {
        Err(eh::PropError::of(eh::PropErrorKind::InsufficientArgs))
    } else {
        Ok(args)
    }
}


/// retrieve the contents of the file as a byte array slice
pub fn get_bytes(file: &str) -> eh::PropResult<Vec<u8>> {
    let reader = BufReader::new(File::open(file)?);

    let mut buffer = Vec::new();

    for byte in reader.bytes() {
        buffer.push(byte.unwrap());
    }

    Ok(buffer)
}
