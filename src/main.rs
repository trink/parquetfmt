// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod parquet {
    include!(concat!(env!("OUT_DIR"), "/parquet.rs"));
}

fn main() {
    let matches = App::new("parquetfmt")
        .version("0.1.0")
        .author("Mike Trinkala <trink@acm.org>")
        .about("Parquet Schema Formatter")
        .arg(
            Arg::with_name("schema")
                .required(true)
                .takes_value(true)
                .index(1)
                .help(
                    "Parquet schema to beautify or a hyphen
                       for stdin",
                ),
        ).get_matches();
    let schema = matches.value_of("schema").unwrap();
    let mut fh: Box<BufRead> = if schema == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(
            File::open(schema).expect("Unable to open the file"),
        ))
    };

    let mut contents = String::new();
    fh.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut depth = 1;
    let r = parquet::grammar(&contents, &mut depth);
    if r.is_err() {
        println!("Parse error: {:?}", r.err());
        std::process::exit(1);
    }
    std::process::exit(0);
}

#[test]
fn full_parse() {
    let fp = "\
message test {
        required float float;
        required double double;

        required int64 int64;
        optional int64 int64o;
        repeated int64 int64r;
        required int32 int32;
        required int96 int96;

        required int64 int64_none (NONE);
        required int64 int64_u8 (UINT_8);
        required int64 int64_u16 (UINT_16);
        required int64 int64_u32 (UINT_32);
        required int64 int64_u64 (UINT_64);
        required int64 int64_8 (INT_8);
        required int64 int64_16 (INT_16);
        required int64 int64_32 (INT_32);
        required int64 int64_64 (INT_64);
        required int64 int64_micros (TIME_MICROS);

        required binary binary;
        required binary binary_enum (ENUM);
        required binary binary_utf8 (UTF8);
        required binary binary_json (JSON);
        required binary binary_bson (BSON);

        required int32 date (DATE);
        required int32 int32_millis (TIME_MILLIS);

        required boolean boolean;

        required fixed_len_byte_array(5) flba;
        required fixed_len_byte_array(12) flba (INTERVAL);

        optional group my_list (LIST) {
        repeated int32 element;
        }

        required group my_map (MAP) {
        repeated group key_value {
        required binary key (UTF8);
        optional int32 value;
        }
        }

        required group range (TUPLE) {
        required double lo;
        required double hi;
        }

        required int32 decimal (DECIMAL(5,0));

        required int64 int64id = 100;
}
";
    let mut depth = 1;
    let r = parquet::grammar(fp, &mut depth);
    assert!(r.is_ok(), r.unwrap());
}

#[test]
fn error_parse() {
    let fp = "message Document { required int16 foo;}";
    let mut depth = 1;
    let r = parquet::grammar(fp, &mut depth);
    assert!(r.is_err());
}
