#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use(expect)]

extern crate expectest;
extern crate bson;

use bson::*;
use expectest::prelude::*;
use std::io::{Cursor, Result};

describe! lib_test {
    describe! string_test {
        describe! success {
            before_each {
                let mut writer = Cursor::new(vec![]);
                let result = "testing".to_string().to_bson(&mut writer);
            }

            it "writes the bytes to the writer" {
            }

            it "returns an ok result" {
                expect!(result).to(be_ok());
            }
        }
    }
}
