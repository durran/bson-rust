#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use(expect)]

extern crate expectest;
extern crate bson;

use expectest::prelude::*;

describe! lib_test {
    it "works" {
        expect!(true).to(be_true());
    }
}
