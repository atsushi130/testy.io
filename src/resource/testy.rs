
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#[derive(Serialize, Deserialize)]
pub struct Testy {
    pub id: i32,
    pub uri: String,
    pub domain: String
}