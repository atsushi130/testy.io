
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket_contrib::JSON;
use super::super::Testy;

#[post("/testy", format="application/json", data="<resource>")]
pub fn post_testy(resource: JSON<Testy>) -> JSON<Testy> {
    resource
}

#[get("/testy")]
pub fn get_testy() -> JSON<Testy> {

    let testy = Testy {
        id: 1,
        uri: "/testy".to_string(),
        domain: "testy.api.io".to_string(),
    };

    JSON(testy)
}