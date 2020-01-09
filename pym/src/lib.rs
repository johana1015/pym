mod types;
pub use types::PymTypes;

/// The Args and Rets should all avaiable for rust types.
/// String <-> PyString
/// Struct <-> PyDict
pub trait Pym<A, R, E>
where
    A: PymTypes<A>,
{
    fn hello_macro();
    fn exec(&mut self, args: A) -> Result<R, E>;
}
