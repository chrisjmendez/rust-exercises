use super::method::Method;
// Rust enums are similar to the algabraic data-types in Haskell
pub struct Request {
    path: String,
    query_string: Option<String>, //Option functions like Null in a type-safe way
    method: Method,
}