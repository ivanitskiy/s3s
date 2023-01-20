#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
)]

mod gen;

mod rust;
mod smithy;

mod dto;
mod error;
mod headers;
mod ops;

use crate::gen::Codegen;

use std::format as f;

fn o<T: ToOwned + ?Sized>(x: &T) -> T::Owned {
    x.to_owned()
}

fn default<T: Default>() -> T {
    T::default()
}

fn main() {
    let model: smithy::Model = {
        let json_path = std::env::args().nth(1).unwrap();
        let json_file = std::fs::read(json_path).unwrap();
        serde_json::from_slice(&json_file).unwrap()
    };
    assert!(model.smithy == "2.0");

    let rust_types = dto::collect_rust_types(&model);
    let ops = ops::collect_operations(&model);

    {
        let path = "crates/s3s/src/dto/generated.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        dto::codegen(&rust_types, &ops, &mut gen);
    }

    {
        let path = "crates/s3s/src/header/names.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        headers::codegen(&model, &mut gen);
    }

    {
        let path = "crates/s3s/src/error/generated.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        error::codegen(&model, &mut gen);
    }

    {
        let path = "crates/s3s/src/ops/generated.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        ops::codegen(&ops, &rust_types, &mut gen);
    }
}
