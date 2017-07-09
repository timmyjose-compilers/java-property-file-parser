use std::collections::HashMap;
use std::str;

use nom::IResult::{Done, Error, Incomplete};
use nom::Needed;

use error_handler as eh;

/// process the given byte array slice and return the parsed result as
/// a `PropResult`
pub fn process(input: &[u8]) -> eh::PropResult<HashMap<&str, &str>> {
    match lines(input) {
        Done(_, res) => return Ok(res),
        Incomplete(Needed::Size(size)) => {
            let err_msg = format!("needed {} more bytes", size);
            
            return Err(eh::PropError::with_specific(eh::PropErrorKind::ParseError, &err_msg));
        },
        Incomplete(Needed::Unknown) =>
            return Err(eh::PropError::with_specific(eh::PropErrorKind::ParseError,
                                         "unknown number of bytes were required")),
        Error(e) =>
            return Err(eh::PropError::with_specific(eh::PropErrorKind::ParseError,
                                                    e.description())),
    }
}

///
/// lines -> line+
///
/// line -> comment | row
/// 
/// comment -> ('#' | '!') TEXT*
///
/// row -> TEXT* (':' | '=') TEXT*
///
/// TEXT -> [^!#]
///

named!(lines< HashMap<&str, &str> >,
    map!(
        many0!(line),
        |acc: Vec<Option<(_, _)>>| {
            let mut res = HashMap::new();
            
            for elem in acc.iter() {
                if elem.is_some() {
                    let elem = elem.unwrap();
                    res.insert(elem.0, elem.1);
                }
            }
            res
        }
    )
);

named!(line< &[u8], Option<(&str, &str)> >,
        alt!(comment | row) 
);


named!(comment<&[u8], Option<(&str, &str)> >,
       do_parse!(
            preceded!(ws!(alt!(char!('#') | char!('!'))), 
                      ws!(take_while!(call!(|c| c != b'\n')))) >>
                      
            (None)
       )
);

named!(row< &[u8], Option<(&str, &str)> >,
        do_parse!(
            key: map_res!(ws!(take_while!(call!(|c| c != b':' && c != b'='))),
                            str::from_utf8) >>
        
            alt!(char!(':') | char!('=')) >>
         
            val : map_res!(ws!(take_while!(call!(|c| c != b'\n'))),
                           str::from_utf8) >>

            (Some((key.trim(), val)))
        )
);

        


