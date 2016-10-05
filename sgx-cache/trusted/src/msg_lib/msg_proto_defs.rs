// This file is generated. Modifications are done for fitting no_std.
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use core_protobuf::Message as Message_imported_for_functions;
use core_protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct CacheMsg {
    // message fields
    msg_type: ::core_protobuf::SingularField<::collections::string::String>,
    msg: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    client_id: ::core::option::Option<u32>,
    mac: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    time: ::core::option::Option<u64>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for CacheMsg {}

impl CacheMsg {
    pub fn new() -> CacheMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static CacheMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<CacheMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CacheMsg,
        };
        unsafe {
            instance.get(|| {
                CacheMsg {
                    msg_type: ::core_protobuf::SingularField::none(),
                    msg: ::core_protobuf::SingularField::none(),
                    client_id: ::core::option::Option::None,
                    mac: ::core_protobuf::SingularField::none(),
                    time: ::core::option::Option::None,
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required string msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type.clear();
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: ::collections::string::String) {
        self.msg_type = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_type(&mut self) -> &mut ::collections::string::String {
        if self.msg_type.is_none() {
            self.msg_type.set_default();
        };
        self.msg_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_type(&mut self) -> ::collections::string::String {
        self.msg_type.take().unwrap_or_else(|| ::collections::string::String::new())
    }

    pub fn get_msg_type(&self) -> &str {
        match self.msg_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bytes msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::collections::vec::Vec<u8>) {
        self.msg = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::collections::vec::Vec<u8> {
        self.msg.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_msg(&self) -> &[u8] {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 client_id = 3;

    pub fn clear_client_id(&mut self) {
        self.client_id = ::core::option::Option::None;
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: u32) {
        self.client_id = ::core::option::Option::Some(v);
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id.unwrap_or(0)
    }

    // optional bytes mac = 4;

    pub fn clear_mac(&mut self) {
        self.mac.clear();
    }

    pub fn has_mac(&self) -> bool {
        self.mac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mac(&mut self, v: ::collections::vec::Vec<u8>) {
        self.mac = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.mac.is_none() {
            self.mac.set_default();
        };
        self.mac.as_mut().unwrap()
    }

    // Take field
    pub fn take_mac(&mut self) -> ::collections::vec::Vec<u8> {
        self.mac.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_mac(&self) -> &[u8] {
        match self.mac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 time = 5;

    pub fn clear_time(&mut self) {
        self.time = ::core::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u64) {
        self.time = ::core::option::Option::Some(v);
    }

    pub fn get_time(&self) -> u64 {
        self.time.unwrap_or(0)
    }
}

impl ::core_protobuf::Message for CacheMsg {
    fn is_initialized(&self) -> bool {
        if self.msg_type.is_none() {
            return false;
        };
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg_type));
                },
                2 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg));
                },
                3 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.client_id = ::core::option::Option::Some(tmp);
                },
                4 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.mac));
                },
                5 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.time = ::core::option::Option::Some(tmp);
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.msg_type.iter() {
            my_size += ::core_protobuf::rt::string_size(1, &value);
        };
        for value in self.msg.iter() {
            my_size += ::core_protobuf::rt::bytes_size(2, &value);
        };
        for value in self.client_id.iter() {
            my_size += ::core_protobuf::rt::value_size(3, *value, ::core_protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mac.iter() {
            my_size += ::core_protobuf::rt::bytes_size(4, &value);
        };
        for value in self.time.iter() {
            my_size += ::core_protobuf::rt::value_size(5, *value, ::core_protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.client_id {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.mac.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.time {
            try!(os.write_uint64(5, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<CacheMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for CacheMsg {
    fn new() -> CacheMsg {
        CacheMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<CacheMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg_type",
                    CacheMsg::has_msg_type,
                    CacheMsg::get_msg_type,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "msg",
                    CacheMsg::has_msg,
                    CacheMsg::get_msg,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_u32_accessor(
                    "client_id",
                    CacheMsg::has_client_id,
                    CacheMsg::get_client_id,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "mac",
                    CacheMsg::has_mac,
                    CacheMsg::get_mac,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_u64_accessor(
                    "time",
                    CacheMsg::has_time,
                    CacheMsg::get_time,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<CacheMsg>(
                    "CacheMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for CacheMsg {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_msg();
        self.clear_client_id();
        self.clear_mac();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for CacheMsg {
    fn eq(&self, other: &CacheMsg) -> bool {
        self.msg_type == other.msg_type &&
        self.msg == other.msg &&
        self.client_id == other.client_id &&
        self.mac == other.mac &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for CacheMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ErrorMsg {
    // message fields
    description: ::core_protobuf::SingularField<::collections::string::String>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for ErrorMsg {}

impl ErrorMsg {
    pub fn new() -> ErrorMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<ErrorMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorMsg,
        };
        unsafe {
            instance.get(|| {
                ErrorMsg {
                    description: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required string description = 1;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::collections::string::String) {
        self.description = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::collections::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::collections::string::String {
        self.description.take().unwrap_or_else(|| ::collections::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::core_protobuf::Message for ErrorMsg {
    fn is_initialized(&self) -> bool {
        if self.description.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.description.iter() {
            my_size += ::core_protobuf::rt::string_size(1, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<ErrorMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for ErrorMsg {
    fn new() -> ErrorMsg {
        ErrorMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<ErrorMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    ErrorMsg::has_description,
                    ErrorMsg::get_description,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<ErrorMsg>(
                    "ErrorMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for ErrorMsg {
    fn clear(&mut self) {
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for ErrorMsg {
    fn eq(&self, other: &ErrorMsg) -> bool {
        self.description == other.description &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for ErrorMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DhaSessionRequest {
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for DhaSessionRequest {}

impl DhaSessionRequest {
    pub fn new() -> DhaSessionRequest {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static DhaSessionRequest {
        static mut instance: ::core_protobuf::lazy::Lazy<DhaSessionRequest> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DhaSessionRequest,
        };
        unsafe {
            instance.get(|| {
                DhaSessionRequest {
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::core_protobuf::Message for DhaSessionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<DhaSessionRequest>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for DhaSessionRequest {
    fn new() -> DhaSessionRequest {
        DhaSessionRequest::new()
    }

    fn descriptor_static(_: ::core::option::Option<DhaSessionRequest>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::collections::vec::Vec::new();
                ::core_protobuf::reflect::MessageDescriptor::new::<DhaSessionRequest>(
                    "DhaSessionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for DhaSessionRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for DhaSessionRequest {
    fn eq(&self, other: &DhaSessionRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for DhaSessionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DhaMsg1 {
    // message fields
    ga: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    targetinfo: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for DhaMsg1 {}

impl DhaMsg1 {
    pub fn new() -> DhaMsg1 {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static DhaMsg1 {
        static mut instance: ::core_protobuf::lazy::Lazy<DhaMsg1> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DhaMsg1,
        };
        unsafe {
            instance.get(|| {
                DhaMsg1 {
                    ga: ::core_protobuf::SingularField::none(),
                    targetinfo: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes ga = 1;

    pub fn clear_ga(&mut self) {
        self.ga.clear();
    }

    pub fn has_ga(&self) -> bool {
        self.ga.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ga(&mut self, v: ::collections::vec::Vec<u8>) {
        self.ga = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ga(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.ga.is_none() {
            self.ga.set_default();
        };
        self.ga.as_mut().unwrap()
    }

    // Take field
    pub fn take_ga(&mut self) -> ::collections::vec::Vec<u8> {
        self.ga.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_ga(&self) -> &[u8] {
        match self.ga.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes targetinfo = 2;

    pub fn clear_targetinfo(&mut self) {
        self.targetinfo.clear();
    }

    pub fn has_targetinfo(&self) -> bool {
        self.targetinfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetinfo(&mut self, v: ::collections::vec::Vec<u8>) {
        self.targetinfo = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetinfo(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.targetinfo.is_none() {
            self.targetinfo.set_default();
        };
        self.targetinfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetinfo(&mut self) -> ::collections::vec::Vec<u8> {
        self.targetinfo.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_targetinfo(&self) -> &[u8] {
        match self.targetinfo.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::core_protobuf::Message for DhaMsg1 {
    fn is_initialized(&self) -> bool {
        if self.ga.is_none() {
            return false;
        };
        if self.targetinfo.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ga));
                },
                2 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.targetinfo));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ga.iter() {
            my_size += ::core_protobuf::rt::bytes_size(1, &value);
        };
        for value in self.targetinfo.iter() {
            my_size += ::core_protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.ga.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.targetinfo.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<DhaMsg1>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for DhaMsg1 {
    fn new() -> DhaMsg1 {
        DhaMsg1::new()
    }

    fn descriptor_static(_: ::core::option::Option<DhaMsg1>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ga",
                    DhaMsg1::has_ga,
                    DhaMsg1::get_ga,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "targetinfo",
                    DhaMsg1::has_targetinfo,
                    DhaMsg1::get_targetinfo,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<DhaMsg1>(
                    "DhaMsg1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for DhaMsg1 {
    fn clear(&mut self) {
        self.clear_ga();
        self.clear_targetinfo();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for DhaMsg1 {
    fn eq(&self, other: &DhaMsg1) -> bool {
        self.ga == other.ga &&
        self.targetinfo == other.targetinfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for DhaMsg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Report {
    // message fields
    report_data: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    misc: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for Report {}

impl Report {
    pub fn new() -> Report {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static Report {
        static mut instance: ::core_protobuf::lazy::Lazy<Report> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Report,
        };
        unsafe {
            instance.get(|| {
                Report {
                    report_data: ::core_protobuf::SingularField::none(),
                    misc: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes report_data = 1;

    pub fn clear_report_data(&mut self) {
        self.report_data.clear();
    }

    pub fn has_report_data(&self) -> bool {
        self.report_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_data(&mut self, v: ::collections::vec::Vec<u8>) {
        self.report_data = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_data(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.report_data.is_none() {
            self.report_data.set_default();
        };
        self.report_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_data(&mut self) -> ::collections::vec::Vec<u8> {
        self.report_data.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_report_data(&self) -> &[u8] {
        match self.report_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes misc = 2;

    pub fn clear_misc(&mut self) {
        self.misc.clear();
    }

    pub fn has_misc(&self) -> bool {
        self.misc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_misc(&mut self, v: ::collections::vec::Vec<u8>) {
        self.misc = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_misc(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.misc.is_none() {
            self.misc.set_default();
        };
        self.misc.as_mut().unwrap()
    }

    // Take field
    pub fn take_misc(&mut self) -> ::collections::vec::Vec<u8> {
        self.misc.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_misc(&self) -> &[u8] {
        match self.misc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::core_protobuf::Message for Report {
    fn is_initialized(&self) -> bool {
        if self.report_data.is_none() {
            return false;
        };
        if self.misc.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.report_data));
                },
                2 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.misc));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.report_data.iter() {
            my_size += ::core_protobuf::rt::bytes_size(1, &value);
        };
        for value in self.misc.iter() {
            my_size += ::core_protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.report_data.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.misc.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<Report>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for Report {
    fn new() -> Report {
        Report::new()
    }

    fn descriptor_static(_: ::core::option::Option<Report>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "report_data",
                    Report::has_report_data,
                    Report::get_report_data,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "misc",
                    Report::has_misc,
                    Report::get_misc,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<Report>(
                    "Report",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for Report {
    fn clear(&mut self) {
        self.clear_report_data();
        self.clear_misc();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for Report {
    fn eq(&self, other: &Report) -> bool {
        self.report_data == other.report_data &&
        self.misc == other.misc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for Report {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DhaMsg2 {
    // message fields
    gb: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    report: ::core_protobuf::SingularPtrField<Report>,
    report_mac: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for DhaMsg2 {}

impl DhaMsg2 {
    pub fn new() -> DhaMsg2 {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static DhaMsg2 {
        static mut instance: ::core_protobuf::lazy::Lazy<DhaMsg2> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DhaMsg2,
        };
        unsafe {
            instance.get(|| {
                DhaMsg2 {
                    gb: ::core_protobuf::SingularField::none(),
                    report: ::core_protobuf::SingularPtrField::none(),
                    report_mac: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes gb = 1;

    pub fn clear_gb(&mut self) {
        self.gb.clear();
    }

    pub fn has_gb(&self) -> bool {
        self.gb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gb(&mut self, v: ::collections::vec::Vec<u8>) {
        self.gb = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gb(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.gb.is_none() {
            self.gb.set_default();
        };
        self.gb.as_mut().unwrap()
    }

    // Take field
    pub fn take_gb(&mut self) -> ::collections::vec::Vec<u8> {
        self.gb.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_gb(&self) -> &[u8] {
        match self.gb.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .Report report = 2;

    pub fn clear_report(&mut self) {
        self.report.clear();
    }

    pub fn has_report(&self) -> bool {
        self.report.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report(&mut self, v: Report) {
        self.report = ::core_protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report(&mut self) -> &mut Report {
        if self.report.is_none() {
            self.report.set_default();
        };
        self.report.as_mut().unwrap()
    }

    // Take field
    pub fn take_report(&mut self) -> Report {
        self.report.take().unwrap_or_else(|| Report::new())
    }

    pub fn get_report(&self) -> &Report {
        self.report.as_ref().unwrap_or_else(|| Report::default_instance())
    }

    // required bytes report_mac = 3;

    pub fn clear_report_mac(&mut self) {
        self.report_mac.clear();
    }

    pub fn has_report_mac(&self) -> bool {
        self.report_mac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_mac(&mut self, v: ::collections::vec::Vec<u8>) {
        self.report_mac = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_mac(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.report_mac.is_none() {
            self.report_mac.set_default();
        };
        self.report_mac.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_mac(&mut self) -> ::collections::vec::Vec<u8> {
        self.report_mac.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_report_mac(&self) -> &[u8] {
        match self.report_mac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::core_protobuf::Message for DhaMsg2 {
    fn is_initialized(&self) -> bool {
        if self.gb.is_none() {
            return false;
        };
        if self.report.is_none() {
            return false;
        };
        if self.report_mac.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gb));
                },
                2 => {
                    try!(::core_protobuf::rt::read_singular_message_into(wire_type, is, &mut self.report));
                },
                3 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.report_mac));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.gb.iter() {
            my_size += ::core_protobuf::rt::bytes_size(1, &value);
        };
        for value in self.report.iter() {
            let len = value.compute_size();
            my_size += 1 + ::core_protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.report_mac.iter() {
            my_size += ::core_protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.gb.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.report.as_ref() {
            try!(os.write_tag(2, ::core_protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.report_mac.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<DhaMsg2>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for DhaMsg2 {
    fn new() -> DhaMsg2 {
        DhaMsg2::new()
    }

    fn descriptor_static(_: ::core::option::Option<DhaMsg2>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "gb",
                    DhaMsg2::has_gb,
                    DhaMsg2::get_gb,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_message_accessor(
                    "report",
                    DhaMsg2::has_report,
                    DhaMsg2::get_report,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "report_mac",
                    DhaMsg2::has_report_mac,
                    DhaMsg2::get_report_mac,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<DhaMsg2>(
                    "DhaMsg2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for DhaMsg2 {
    fn clear(&mut self) {
        self.clear_gb();
        self.clear_report();
        self.clear_report_mac();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for DhaMsg2 {
    fn eq(&self, other: &DhaMsg2) -> bool {
        self.gb == other.gb &&
        self.report == other.report &&
        self.report_mac == other.report_mac &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for DhaMsg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DhaMsg3 {
    // message fields
    report: ::core_protobuf::SingularPtrField<Report>,
    report_mac: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for DhaMsg3 {}

impl DhaMsg3 {
    pub fn new() -> DhaMsg3 {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static DhaMsg3 {
        static mut instance: ::core_protobuf::lazy::Lazy<DhaMsg3> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DhaMsg3,
        };
        unsafe {
            instance.get(|| {
                DhaMsg3 {
                    report: ::core_protobuf::SingularPtrField::none(),
                    report_mac: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Report report = 1;

    pub fn clear_report(&mut self) {
        self.report.clear();
    }

    pub fn has_report(&self) -> bool {
        self.report.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report(&mut self, v: Report) {
        self.report = ::core_protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report(&mut self) -> &mut Report {
        if self.report.is_none() {
            self.report.set_default();
        };
        self.report.as_mut().unwrap()
    }

    // Take field
    pub fn take_report(&mut self) -> Report {
        self.report.take().unwrap_or_else(|| Report::new())
    }

    pub fn get_report(&self) -> &Report {
        self.report.as_ref().unwrap_or_else(|| Report::default_instance())
    }

    // required bytes report_mac = 2;

    pub fn clear_report_mac(&mut self) {
        self.report_mac.clear();
    }

    pub fn has_report_mac(&self) -> bool {
        self.report_mac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_mac(&mut self, v: ::collections::vec::Vec<u8>) {
        self.report_mac = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_mac(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.report_mac.is_none() {
            self.report_mac.set_default();
        };
        self.report_mac.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_mac(&mut self) -> ::collections::vec::Vec<u8> {
        self.report_mac.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_report_mac(&self) -> &[u8] {
        match self.report_mac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::core_protobuf::Message for DhaMsg3 {
    fn is_initialized(&self) -> bool {
        if self.report.is_none() {
            return false;
        };
        if self.report_mac.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_message_into(wire_type, is, &mut self.report));
                },
                2 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.report_mac));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.report.iter() {
            let len = value.compute_size();
            my_size += 1 + ::core_protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.report_mac.iter() {
            my_size += ::core_protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.report.as_ref() {
            try!(os.write_tag(1, ::core_protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.report_mac.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<DhaMsg3>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for DhaMsg3 {
    fn new() -> DhaMsg3 {
        DhaMsg3::new()
    }

    fn descriptor_static(_: ::core::option::Option<DhaMsg3>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_message_accessor(
                    "report",
                    DhaMsg3::has_report,
                    DhaMsg3::get_report,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "report_mac",
                    DhaMsg3::has_report_mac,
                    DhaMsg3::get_report_mac,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<DhaMsg3>(
                    "DhaMsg3",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for DhaMsg3 {
    fn clear(&mut self) {
        self.clear_report();
        self.clear_report_mac();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for DhaMsg3 {
    fn eq(&self, other: &DhaMsg3) -> bool {
        self.report == other.report &&
        self.report_mac == other.report_mac &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for DhaMsg3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BoolMsg {
    // message fields
    val: ::core::option::Option<bool>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for BoolMsg {}

impl BoolMsg {
    pub fn new() -> BoolMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static BoolMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<BoolMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BoolMsg,
        };
        unsafe {
            instance.get(|| {
                BoolMsg {
                    val: ::core::option::Option::None,
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool val = 1;

    pub fn clear_val(&mut self) {
        self.val = ::core::option::Option::None;
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: bool) {
        self.val = ::core::option::Option::Some(v);
    }

    pub fn get_val(&self) -> bool {
        self.val.unwrap_or(false)
    }
}

impl ::core_protobuf::Message for BoolMsg {
    fn is_initialized(&self) -> bool {
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.val = ::core::option::Option::Some(tmp);
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.val.is_some() {
            my_size += 2;
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.val {
            try!(os.write_bool(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<BoolMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for BoolMsg {
    fn new() -> BoolMsg {
        BoolMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<BoolMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_bool_accessor(
                    "val",
                    BoolMsg::has_val,
                    BoolMsg::get_val,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<BoolMsg>(
                    "BoolMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for BoolMsg {
    fn clear(&mut self) {
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for BoolMsg {
    fn eq(&self, other: &BoolMsg) -> bool {
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for BoolMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct U8Msg {
    // message fields
    val: ::core::option::Option<u32>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for U8Msg {}

impl U8Msg {
    pub fn new() -> U8Msg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static U8Msg {
        static mut instance: ::core_protobuf::lazy::Lazy<U8Msg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const U8Msg,
        };
        unsafe {
            instance.get(|| {
                U8Msg {
                    val: ::core::option::Option::None,
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 val = 1;

    pub fn clear_val(&mut self) {
        self.val = ::core::option::Option::None;
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: u32) {
        self.val = ::core::option::Option::Some(v);
    }

    pub fn get_val(&self) -> u32 {
        self.val.unwrap_or(0)
    }
}

impl ::core_protobuf::Message for U8Msg {
    fn is_initialized(&self) -> bool {
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.val = ::core::option::Option::Some(tmp);
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.val.iter() {
            my_size += ::core_protobuf::rt::value_size(1, *value, ::core_protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.val {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<U8Msg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for U8Msg {
    fn new() -> U8Msg {
        U8Msg::new()
    }

    fn descriptor_static(_: ::core::option::Option<U8Msg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_u32_accessor(
                    "val",
                    U8Msg::has_val,
                    U8Msg::get_val,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<U8Msg>(
                    "U8Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for U8Msg {
    fn clear(&mut self) {
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for U8Msg {
    fn eq(&self, other: &U8Msg) -> bool {
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for U8Msg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct U32Msg {
    // message fields
    val: ::core::option::Option<u32>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for U32Msg {}

impl U32Msg {
    pub fn new() -> U32Msg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static U32Msg {
        static mut instance: ::core_protobuf::lazy::Lazy<U32Msg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const U32Msg,
        };
        unsafe {
            instance.get(|| {
                U32Msg {
                    val: ::core::option::Option::None,
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 val = 1;

    pub fn clear_val(&mut self) {
        self.val = ::core::option::Option::None;
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: u32) {
        self.val = ::core::option::Option::Some(v);
    }

    pub fn get_val(&self) -> u32 {
        self.val.unwrap_or(0)
    }
}

impl ::core_protobuf::Message for U32Msg {
    fn is_initialized(&self) -> bool {
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.val = ::core::option::Option::Some(tmp);
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.val.iter() {
            my_size += ::core_protobuf::rt::value_size(1, *value, ::core_protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.val {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<U32Msg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for U32Msg {
    fn new() -> U32Msg {
        U32Msg::new()
    }

    fn descriptor_static(_: ::core::option::Option<U32Msg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_u32_accessor(
                    "val",
                    U32Msg::has_val,
                    U32Msg::get_val,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<U32Msg>(
                    "U32Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for U32Msg {
    fn clear(&mut self) {
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for U32Msg {
    fn eq(&self, other: &U32Msg) -> bool {
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for U32Msg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BytesMsg {
    // message fields
    val: ::core_protobuf::SingularField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for BytesMsg {}

impl BytesMsg {
    pub fn new() -> BytesMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static BytesMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<BytesMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BytesMsg,
        };
        unsafe {
            instance.get(|| {
                BytesMsg {
                    val: ::core_protobuf::SingularField::none(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes val = 1;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::collections::vec::Vec<u8>) {
        self.val = ::core_protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::collections::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::collections::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::collections::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::core_protobuf::Message for BytesMsg {
    fn is_initialized(&self) -> bool {
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.val.iter() {
            my_size += ::core_protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<BytesMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for BytesMsg {
    fn new() -> BytesMsg {
        BytesMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<BytesMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    BytesMsg::has_val,
                    BytesMsg::get_val,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<BytesMsg>(
                    "BytesMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for BytesMsg {
    fn clear(&mut self) {
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for BytesMsg {
    fn eq(&self, other: &BytesMsg) -> bool {
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for BytesMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BytesVecMsg {
    // message fields
    val: ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for BytesVecMsg {}

impl BytesVecMsg {
    pub fn new() -> BytesVecMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static BytesVecMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<BytesVecMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BytesVecMsg,
        };
        unsafe {
            instance.get(|| {
                BytesVecMsg {
                    val: ::core_protobuf::RepeatedField::new(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes val = 1;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>>) {
        self.val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_val(&mut self) -> &mut ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>> {
        &mut self.val
    }

    // Take field
    pub fn take_val(&mut self) -> ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>> {
        ::core::mem::replace(&mut self.val, ::core_protobuf::RepeatedField::new())
    }

    pub fn get_val(&self) -> &[::collections::vec::Vec<u8>] {
        &self.val
    }
}

impl ::core_protobuf::Message for BytesVecMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::core_protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.val.iter() {
            my_size += ::core_protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        for v in self.val.iter() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<BytesVecMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for BytesVecMsg {
    fn new() -> BytesVecMsg {
        BytesVecMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<BytesVecMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "val",
                    BytesVecMsg::get_val,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<BytesVecMsg>(
                    "BytesVecMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for BytesVecMsg {
    fn clear(&mut self) {
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for BytesVecMsg {
    fn eq(&self, other: &BytesVecMsg) -> bool {
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for BytesVecMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SubCacheMsg {
    // message fields
    number: ::core::option::Option<u32>,
    filters: ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::core_protobuf::UnknownFields,
    cached_size: ::core::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::core::marker::Sync for SubCacheMsg {}

impl SubCacheMsg {
    pub fn new() -> SubCacheMsg {
        ::core::default::Default::default()
    }

    pub fn default_instance() -> &'static SubCacheMsg {
        static mut instance: ::core_protobuf::lazy::Lazy<SubCacheMsg> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubCacheMsg,
        };
        unsafe {
            instance.get(|| {
                SubCacheMsg {
                    number: ::core::option::Option::None,
                    filters: ::core_protobuf::RepeatedField::new(),
                    unknown_fields: ::core_protobuf::UnknownFields::new(),
                    cached_size: ::core::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 number = 1;

    pub fn clear_number(&mut self) {
        self.number = ::core::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: u32) {
        self.number = ::core::option::Option::Some(v);
    }

    pub fn get_number(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    // repeated bytes filters = 2;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>>) {
        self.filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filters(&mut self) -> &mut ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>> {
        &mut self.filters
    }

    // Take field
    pub fn take_filters(&mut self) -> ::core_protobuf::RepeatedField<::collections::vec::Vec<u8>> {
        ::core::mem::replace(&mut self.filters, ::core_protobuf::RepeatedField::new())
    }

    pub fn get_filters(&self) -> &[::collections::vec::Vec<u8>] {
        &self.filters
    }
}

impl ::core_protobuf::Message for SubCacheMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::core_protobuf::CodedInputStream) -> ::core_protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::core_protobuf::wire_format::WireTypeVarint {
                        return ::core::result::Result::Err(::core_protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.number = ::core::option::Option::Some(tmp);
                },
                2 => {
                    try!(::core_protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.filters));
                },
                _ => {
                    try!(::core_protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::core::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.number.iter() {
            my_size += ::core_protobuf::rt::value_size(1, *value, ::core_protobuf::wire_format::WireTypeVarint);
        };
        for value in self.filters.iter() {
            my_size += ::core_protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::core_protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::core_protobuf::CodedOutputStream) -> ::core_protobuf::ProtobufResult<()> {
        if let Some(v) = self.number {
            try!(os.write_uint32(1, v));
        };
        for v in self.filters.iter() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::core::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::core_protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::core_protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::core::any::TypeId {
        ::core::any::TypeId::of::<SubCacheMsg>()
    }

    fn as_any(&self) -> &::core::any::Any {
        self as &::core::any::Any
    }

    fn descriptor(&self) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        ::core_protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::core_protobuf::MessageStatic for SubCacheMsg {
    fn new() -> SubCacheMsg {
        SubCacheMsg::new()
    }

    fn descriptor_static(_: ::core::option::Option<SubCacheMsg>) -> &'static ::core_protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::core_protobuf::lazy::Lazy<::core_protobuf::reflect::MessageDescriptor> = ::core_protobuf::lazy::Lazy {
            lock: ::core_protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::core_protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::collections::vec::Vec::new();
                fields.push(::core_protobuf::reflect::accessor::make_singular_u32_accessor(
                    "number",
                    SubCacheMsg::has_number,
                    SubCacheMsg::get_number,
                ));
                fields.push(::core_protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "filters",
                    SubCacheMsg::get_filters,
                ));
                ::core_protobuf::reflect::MessageDescriptor::new::<SubCacheMsg>(
                    "SubCacheMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::core_protobuf::Clear for SubCacheMsg {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_filters();
        self.unknown_fields.clear();
    }
}

impl ::core::cmp::PartialEq for SubCacheMsg {
    fn eq(&self, other: &SubCacheMsg) -> bool {
        self.number == other.number &&
        self.filters == other.filters &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::core::fmt::Debug for SubCacheMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core_protobuf::text_format::fmt(self, f)
    }
}


static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x73, 0x72, 0x63, 0x2f, 0x6d, 0x73, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f,
    0x64, 0x65, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x57, 0x0a, 0x08, 0x43, 0x61,
    0x63, 0x68, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x61, 0x63, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x04, 0x22, 0x1f, 0x0a, 0x08, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x73, 0x67, 0x12,
    0x13, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x22, 0x13, 0x0a, 0x11, 0x44, 0x68, 0x61, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x29, 0x0a, 0x07, 0x44, 0x68, 0x61,
    0x4d, 0x73, 0x67, 0x31, 0x12, 0x0a, 0x0a, 0x02, 0x67, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0x2b, 0x0a, 0x06, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x13,
    0x0a, 0x0b, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x6d, 0x69, 0x73, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x0c, 0x22, 0x42, 0x0a, 0x07, 0x44, 0x68, 0x61, 0x4d, 0x73, 0x67, 0x32, 0x12, 0x0a, 0x0a, 0x02,
    0x67, 0x62, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x17, 0x0a, 0x06, 0x72, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x52, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x63, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x36, 0x0a, 0x07, 0x44, 0x68, 0x61, 0x4d, 0x73, 0x67, 0x33,
    0x12, 0x17, 0x0a, 0x06, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x07, 0x2e, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x63, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x16, 0x0a,
    0x07, 0x42, 0x6f, 0x6f, 0x6c, 0x4d, 0x73, 0x67, 0x12, 0x0b, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x08, 0x22, 0x14, 0x0a, 0x05, 0x55, 0x38, 0x4d, 0x73, 0x67, 0x12, 0x0b,
    0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x15, 0x0a, 0x06, 0x55,
    0x33, 0x32, 0x4d, 0x73, 0x67, 0x12, 0x0b, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0d, 0x22, 0x17, 0x0a, 0x08, 0x42, 0x79, 0x74, 0x65, 0x73, 0x4d, 0x73, 0x67, 0x12, 0x0b,
    0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x1a, 0x0a, 0x0b, 0x42,
    0x79, 0x74, 0x65, 0x73, 0x56, 0x65, 0x63, 0x4d, 0x73, 0x67, 0x12, 0x0b, 0x0a, 0x03, 0x76, 0x61,
    0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x2e, 0x0a, 0x0b, 0x53, 0x75, 0x62, 0x43, 0x61,
    0x63, 0x68, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x0e, 0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x4a, 0xae, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x01, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x01, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x01, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x12,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x02, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x02, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x03, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x03, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x04, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x04,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x11, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x04, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x05, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x05, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x05, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x0f, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x10, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x10, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x11,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x14, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x14,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x19, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x19, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x1b, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1c, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1c, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x1f, 0x00, 0x22, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1b, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x21, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x21, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x21, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x24, 0x00, 0x26,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x24, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x25, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x25, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25,
    0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x18, 0x19, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x2d, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x03, 0x30, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12,
    0x04, 0x34, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x34, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x35, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x35, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x38, 0x00,
    0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x38, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x39, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x39, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x39, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x1b, 0x1c,
];

static mut file_descriptor_proto_lazy: ::core_protobuf::lazy::Lazy<::core_protobuf::descriptor::FileDescriptorProto> = ::core_protobuf::lazy::Lazy {
    lock: ::core_protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::core_protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::core_protobuf::descriptor::FileDescriptorProto {
    ::core_protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::core_protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
