// #![feature(plugin)]
// #![cfg_attr(test, plugin(stainless))]
// #[macro_use(expect)]

// extern crate expectest;
// extern crate bson;

// use bson::*;
// use expectest::prelude::*;
// use std::io::Cursor;

// describe! string_test {
    // describe! serialization {
        // describe! success {
            // before_each {
                // let mut writer = Cursor::new(vec![]);
                // let expected = vec![8, 0, 0, 0, 116, 101, 115, 116, 105, 110, 103, 0];
                // let result = "testing".to_string().to_bson(&mut writer);
            // }

            // it "returns an ok result" {
                // expect!(result).to(be_ok());
            // }

            // it "writes the bytes to the writer" {
                // expect!(writer.into_inner()).to(be_equal_to(expected));
            // }
        // }
    // }

    // describe! deserialization {
        // describe! success {
            // before_each {
                // let bytes = vec![8, 0, 0, 0, 116, 101, 115, 116, 105, 110, 103, 0];
                // let mut reader = Cursor::new(bytes);
                // let result = String::from_bson(&mut reader);
            // }

            // it "returns an ok result" {
                // expect!(result).to(be_ok());
            // }

            // it "reads the bytes from the reader" {
                // expect!(result).to(be_ok().value("testing"));
            // }
        // }
    // }
// }
