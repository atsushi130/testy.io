
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;

mod resource;
pub use resource::Testy;

mod controller;

fn main() {
    controller::routes();
}
