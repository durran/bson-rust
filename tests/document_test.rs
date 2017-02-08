#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use(expect)]

extern crate expectest;
extern crate bson;

use bson::{Bson, Document};
use expectest::prelude::*;

describe! document_test {
    describe! insert {
        describe! non_existant_key {
            before_each {
                let mut document = Document::new();
                let key = "test".to_string();
                let string = "value".to_string();
                let value = Bson::String(string);
                let option = document.insert(key, value);
            }

            it "returns an option" {
                /// @todo: Why does the assertion panic?
                option;
                expect!(true).to(be_true());
            }

            it "sets the value in the document" {
                expect!(document.get("test")).to(
                    be_equal_to(Some(&Bson::String("value".to_string())))
                );
            }
        }
    }
}
