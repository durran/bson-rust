#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use(expect)]
extern crate expectest;

#[macro_use]
extern crate bson;

use bson::{Bson, Document};
use expectest::prelude::*;

describe! document_test {
    describe! document {
        before_each {
            let document = document! {
                "double" => 24.5,
                "string" => "value"
            };
        }

        it "handles double values" {
            expect!(document.get("double")).to(
                be_equal_to(Some(&Bson::Double(24.5)))
            );
        }

        it "handles string values" {
            expect!(document.get("string")).to(
                be_equal_to(Some(&Bson::String("value".to_string())))
            );
        }
    }

    describe! get {
        describe! when_the_key_does_not_exist {
            before_each {
                let document = document! {};
            }

            it "returns none" {
                expect!(document.get("test")).to(be_none());
            }
        }

        describe! when_the_key_exists {
            before_each {
                let document = document! {
                    "test" => "value"
                };
            }

            it "returns the some option" {
                expect!(document.get("test")).to(
                    be_equal_to(Some(&Bson::String("value".to_string())))
                );
            }
        }
    }

    describe! insert {
        describe! when_the_key_does_not_exist {
            before_each {
                let mut document = Document::new();
                let key = "test".to_string();
                let string = "value".to_string();
                let value = Bson::String(string);
                let option = document.insert(key, value);
            }

            it "returns the old value" {
                expect!(option).to(be_none());
            }

            it "sets the value in the document" {
                expect!(document.get("test")).to(
                    be_equal_to(Some(&Bson::String("value".to_string())))
                );
            }
        }

        describe! when_the_key_exists {
            before_each {
                let mut document = document! {
                    "test" => "value"
                };
                let new_key = "test".to_string();
                let new_string = "values".to_string();
                let new_value = Bson::String(new_string);
                let option = document.insert(new_key, new_value);
            }

            it "returns the old value" {
                expect!(option).to(
                    be_equal_to(Some(Bson::String("value".to_string())))
                );
            }

            it "replaces the value in the document" {
                expect!(document.get("test")).to(
                    be_equal_to(Some(&Bson::String("values".to_string())))
                );
            }
        }
    }
}