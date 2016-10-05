#![crate_type = "lib"]

#![no_std]
#![feature(collections,const_fn,slice_concat_ext)]

#[macro_use] extern crate collections;
extern crate core_collections;
extern crate core_io as io;
extern crate spin;

pub use unknown::UnknownFields;
// pub use unknown::UnknownFieldsIter;
// pub use unknown::UnknownValue;
// pub use unknown::UnknownValueRef;
// pub use unknown::UnknownValues;
// pub use unknown::UnknownValuesIter;
pub use repeated::RepeatedField;
pub use singular::SingularField;
pub use singular::SingularPtrField;
pub use clear::Clear;
pub use pbcore::Message;
pub use pbcore::MessageStatic;
pub use pbcore::ProtobufEnum;
pub use pbcore::parse_from_bytes;
// pub use pbcore::parse_from_reader;
// pub use pbcore::parse_length_delimited_from;
// pub use pbcore::parse_length_delimited_from_bytes;
pub use stream::CodedInputStream;
pub use stream::CodedOutputStream;
pub use stream::wire_format;
pub use error::ProtobufResult;
pub use error::ProtobufError;

// generated
pub mod descriptor;

pub mod pbcore;
pub mod rt;
pub mod lazy;
pub mod repeated;
pub mod singular;
pub mod clear;
pub mod reflect;
pub mod text_format;
pub mod stream;
pub mod error;

// used by test
pub mod hex;

// used by rust-grpc
pub mod descriptorx;

mod unknown;
mod strx;
mod rust;

// so `use protobuf::*` could work in descriptor mod
mod protobuf {
    pub use descriptor;
    pub use descriptorx;
    pub use reflect;
    pub use pbcore::*;
    pub use error::*;
    pub use stream::*;
    pub use rt;
    pub use text_format;
    pub use lazy;
    pub use unknown::UnknownFields;
//     pub use unknown::UnknownFieldsIter;
//     pub use unknown::UnknownValue;
//     pub use unknown::UnknownValueRef;
//     pub use unknown::UnknownValues;
//     pub use unknown::UnknownValuesIter;
    pub use repeated::RepeatedField;
    pub use singular::SingularField;
    pub use singular::SingularPtrField;
    pub use clear::Clear;
}
