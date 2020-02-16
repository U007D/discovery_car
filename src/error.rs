/// The set of errors returnable by this crate.  Note that functions which can truly only fail
/// for one reason should generally return `Option<T>`, not `Result<T>`.
#[derive(Debug)]
pub enum Error {
    SomeError,
}
