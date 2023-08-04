#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod communication;
mod datatypes;
mod dataunits;
mod datavalues;
mod layouts;

pub use app::DisplayApplication;
use datavalue_derive::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct Foo;
#[derive(MyTrait)]
#[my_trait(answer = 0)]
struct Bar;

#[test]
fn default() {
    assert_eq!(Foo::answer(), 42);
}
#[test]
fn getter() {
    assert_eq!(Bar::answer(), 0);
}