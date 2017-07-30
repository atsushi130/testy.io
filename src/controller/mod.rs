
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

mod testy_controller;
use rocket;

pub fn routes() {
    rocket::ignite().mount("/", routes![testy_controller::get_testy, testy_controller::post_testy]).launch();
}