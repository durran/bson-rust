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
                "string" => "value",
                "document" => {},
                "document_with_vals" => {
                    "embed_with_vals" => {
                        "test" => "value"
                    }
                },
                "array" => [],
                "array_with_vals" => [ 1, 2, 3 ],
                "array_with_one" => [ 1 ],
                "binary" => (bson_binary!(1, vec![1, 1, 1])),
                "undefined" => (bson_undefined!()),
                "true" => true,
                "false" => false,
                "datetime" => (bson_datetime!(1486564200000)),
                "null" => (bson_null!()),
                "regexp" => (bson_regexp!("/test/", "i")),
                "code" => (bson_code!("foo = bar", document!())),
                "symbol" => (bson_symbol!("test")),
                "int8" => 42i8,
                "int16" => 42i16,
                "int32" => 42,
                "timestamp" => 1000u64,
                "int64" => 42i64,
                "minkey" => (bson_minkey!()),
                "maxkey" => (bson_maxkey!())
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

        it "handles empty embedded documents" {
            expect!(document.get("document")).to(
                be_equal_to(Some(&Bson::Document(document!())))
            );
        }

        it "handles multi-level embedded documents" {
            expect!(document.get("document_with_vals")).to(
                be_equal_to(Some(&Bson::Document(document! {
                    "embed_with_vals" => {
                        "test" => "value"
                    }
                })))
            );
        }

        it "handles empty arrays" {
            expect!(document.get("array")).to(
                be_equal_to(Some(&Bson::Array(vec![])))
            );
        }

        it "handles arrays with values" {
            expect!(document.get("array_with_vals")).to(be_equal_to(
                Some(&Bson::Array(vec![Bson::Int32(1), Bson::Int32(2), Bson::Int32(3)]))
            ));
        }

        it "handles arrays with one value" {
            expect!(document.get("array_with_one")).to(be_equal_to(
                Some(&Bson::Array(vec![Bson::Int32(1)]))
            ));
        }

        it "handles binary values" {
            expect!(document.get("binary")).to(be_equal_to(
                Some(&Bson::Binary(1u8, vec![1u8, 1u8, 1u8]))
            ));
        }

        it "handles undefined" {
            expect!(document.get("undefined")).to(
                be_equal_to(Some(&Bson::Undefined))
            );
        }

        it "handles boolean true" {
            expect!(document.get("true")).to(
                be_equal_to(Some(&Bson::Boolean(true)))
            );
        }

        it "handles boolean false" {
            expect!(document.get("false")).to(
                be_equal_to(Some(&Bson::Boolean(false)))
            );
        }

        it "handles datetime values" {
            expect!(document.get("datetime")).to(
                be_equal_to(Some(&Bson::DateTime(1486564200000)))
            );
        }

        it "handles null" {
            expect!(document.get("null")).to(
                be_equal_to(Some(&Bson::Null))
            );
        }

        it "handles regexp values" {
            expect!(document.get("regexp")).to(
                be_equal_to(Some(&Bson::RegExp("/test/".to_string(), "i".to_string())))
            );
        }

        it "handles code values" {
            expect!(document.get("code")).to(
                be_equal_to(Some(&Bson::Code("foo = bar".to_string(), document!())))
            );
        }

        it "handles symbol values" {
            expect!(document.get("symbol")).to(
                be_equal_to(Some(&Bson::Symbol("test".to_string())))
            );
        }

        it "handles 8bit integers" {
            expect!(document.get("int8")).to(
                be_equal_to(Some(&Bson::Int32(42)))
            );
        }

        it "handles 16bit integers" {
            expect!(document.get("int16")).to(
                be_equal_to(Some(&Bson::Int32(42)))
            );
        }

        it "handles 32bit integers" {
            expect!(document.get("int32")).to(
                be_equal_to(Some(&Bson::Int32(42)))
            );
        }

        it "handles timestamps" {
            expect!(document.get("timestamp")).to(
                be_equal_to(Some(&Bson::Timestamp(1000u64)))
            );
        }

        it "handles 64bit integers" {
            expect!(document.get("int64")).to(
                be_equal_to(Some(&Bson::Int64(42i64)))
            );
        }

        it "handles min key" {
            expect!(document.get("minkey")).to(be_equal_to(Some(&Bson::MinKey)));
        }

        it "handles max key" {
            expect!(document.get("maxkey")).to(be_equal_to(Some(&Bson::MaxKey)));
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
