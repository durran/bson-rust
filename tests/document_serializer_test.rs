#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use(expect)]

extern crate expectest;
extern crate bson;

use bson::{Document, DocumentSerializer};
use expectest::prelude::*;
use std::io::Cursor;

describe! document_serializer_test {
    describe! serialize {
        describe! string {
            describe! success {
                before_each {
                    let mut writer = Cursor::new(vec![]);
                    let mut serializer = DocumentSerializer::new(&mut writer);
                    let document = Document::new();
                    let result = serializer.serialize(&document);
                }

                it "returns an ok result" {
                    expect!(result).to(be_ok());
                }
            }
        }
    }
}
