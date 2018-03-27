#![recursion_limit = "128"]

mod game;
mod web;

extern crate rand;
#[macro_use]
extern crate yew;

fn main() {
    web::serve();
}
