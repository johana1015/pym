#[macro_use]
extern crate pym_derive;
use pym::Pym;

#[derive(Pym)]
struct Filter;

fn main() {
    Filter::hello_macro();
}
