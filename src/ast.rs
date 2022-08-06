// use std::fmt::{self, Write};

// use crate::{small_string::SmallString, Error, Result};

// pub enum Function {
//     Builtin(SmallString),
//     Lambda { formals: Value, body: Value },
// }

// #[derive(Debug)]
// pub enum Value {
//     Number(i64),
//     Symbol(SmallString),
//     SExpression(Vec<Value>),
//     QExpression(Vec<Value>),
// }

// impl Value {
//     pub fn push(&mut self, value: Self) -> Result<()> {
//         match self {
//             Value::QExpression(elements)
//             | Value::SExpression(elements) => {
//                 elements.push(value);
//                 Ok(())
//             }
//             _ => Err(Error::CantAddToValue(value)),
//         }
//     }

//     pub fn pop(&mut self) -> Result<Value> {
//         match self {
//             Value::QExpression(elements)
//             | Value::SExpression(elements) => {
//                 elements.pop().ok_or(Error::EmptyCollection)
//             }
//             _ => Err(Error::CantPopFromValue),
//         }
//     }
// }

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fn write_slice(
//             f: &mut fmt::Formatter<'_>,
//             values: &[Value],
//         ) -> fmt::Result {
//             if let Some((last, values)) = values.split_last()
// {                 for value in values {
//                     write!(f, "{value} ")?;
//                 }
//                 write!(f, "{last}")?;
//             }

//             Ok(())
//         }

//         match self {
//             Value::Number(n) => write!(f, "{n}"),
//             Value::Symbol(symbol) => {
//                 f.write_str(symbol.as_str())
//             }
//             Value::SExpression(children) => {
//                 f.write_char('(')?;
//                 write_slice(f, children)?;
//                 f.write_char('(')
//             }
//             Value::QExpression(children) => {
//                 f.write_str("{{")?;
//                 write_slice(f, children)?;
//                 f.write_str("}}")
//             }
//         }
//     }
// }
