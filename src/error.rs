pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("Tried to add {0} into a Value which is not a
    // collection")] CantAddToValue(Value),
    // #[error("Tried to pop an element from a Value which is
    // not a collection")] CantPopFromValue,
    #[error("Tried to pop from an empty collection")]
    EmptyCollection,
    #[error(
        "Type error: Expected {expected}, received {received}"
    )]
    TypeMismatch {
        expected: &'static str,
        received: &'static str,
    },
}
