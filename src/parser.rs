use nom::error::VerboseError;

mod atom;

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;

// #[cfg(test)]
// mod tests {
//     use nom::Finish;

//     use crate::parser::{parse_builtin, parse_operator};

//     #[test]
//     fn odaskod() {
//         println!(
//             "{}",
//             parse_operator("aa").finish().unwrap_err()
//         );
//     }
// }
