#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use(expect)]

extern crate expectest;
extern crate bson;

use bson::*;
use expectest::prelude::*;
use std::io::Cursor;

describe! type_serializer_test {
}
