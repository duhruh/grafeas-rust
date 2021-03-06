// This file is generated by rust-protobuf 2.6.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Deployable {
    // message fields
    pub resource_uri: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Deployable {
    fn default() -> &'a Deployable {
        <Deployable as ::protobuf::Message>::default_instance()
    }
}

impl Deployable {
    pub fn new() -> Deployable {
        ::std::default::Default::default()
    }

    // repeated string resource_uri = 1;


    pub fn get_resource_uri(&self) -> &[::std::string::String] {
        &self.resource_uri
    }
    pub fn clear_resource_uri(&mut self) {
        self.resource_uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_uri(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.resource_uri = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_uri(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resource_uri
    }

    // Take field
    pub fn take_resource_uri(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.resource_uri, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Deployable {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resource_uri)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.resource_uri {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.resource_uri {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Deployable {
        Deployable::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_uri",
                    |m: &Deployable| { &m.resource_uri },
                    |m: &mut Deployable| { &mut m.resource_uri },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Deployable>(
                    "Deployable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Deployable {
        static mut instance: ::protobuf::lazy::Lazy<Deployable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Deployable,
        };
        unsafe {
            instance.get(Deployable::new)
        }
    }
}

impl ::protobuf::Clear for Deployable {
    fn clear(&mut self) {
        self.resource_uri.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Deployable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Deployable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Details {
    // message fields
    pub deployment: ::protobuf::SingularPtrField<Deployment>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Details {
    fn default() -> &'a Details {
        <Details as ::protobuf::Message>::default_instance()
    }
}

impl Details {
    pub fn new() -> Details {
        ::std::default::Default::default()
    }

    // .grafeas.v1beta1.deployment.Deployment deployment = 1;


    pub fn get_deployment(&self) -> &Deployment {
        self.deployment.as_ref().unwrap_or_else(|| Deployment::default_instance())
    }
    pub fn clear_deployment(&mut self) {
        self.deployment.clear();
    }

    pub fn has_deployment(&self) -> bool {
        self.deployment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deployment(&mut self, v: Deployment) {
        self.deployment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deployment(&mut self) -> &mut Deployment {
        if self.deployment.is_none() {
            self.deployment.set_default();
        }
        self.deployment.as_mut().unwrap()
    }

    // Take field
    pub fn take_deployment(&mut self) -> Deployment {
        self.deployment.take().unwrap_or_else(|| Deployment::new())
    }
}

impl ::protobuf::Message for Details {
    fn is_initialized(&self) -> bool {
        for v in &self.deployment {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deployment)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.deployment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.deployment.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Details {
        Details::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Deployment>>(
                    "deployment",
                    |m: &Details| { &m.deployment },
                    |m: &mut Details| { &mut m.deployment },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Details>(
                    "Details",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Details {
        static mut instance: ::protobuf::lazy::Lazy<Details> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Details,
        };
        unsafe {
            instance.get(Details::new)
        }
    }
}

impl ::protobuf::Clear for Details {
    fn clear(&mut self) {
        self.deployment.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Details {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Details {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Deployment {
    // message fields
    pub user_email: ::std::string::String,
    pub deploy_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub undeploy_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub config: ::std::string::String,
    pub address: ::std::string::String,
    pub resource_uri: ::protobuf::RepeatedField<::std::string::String>,
    pub platform: Deployment_Platform,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Deployment {
    fn default() -> &'a Deployment {
        <Deployment as ::protobuf::Message>::default_instance()
    }
}

impl Deployment {
    pub fn new() -> Deployment {
        ::std::default::Default::default()
    }

    // string user_email = 1;


    pub fn get_user_email(&self) -> &str {
        &self.user_email
    }
    pub fn clear_user_email(&mut self) {
        self.user_email.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_email(&mut self, v: ::std::string::String) {
        self.user_email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_email(&mut self) -> &mut ::std::string::String {
        &mut self.user_email
    }

    // Take field
    pub fn take_user_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_email, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp deploy_time = 2;


    pub fn get_deploy_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.deploy_time.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_deploy_time(&mut self) {
        self.deploy_time.clear();
    }

    pub fn has_deploy_time(&self) -> bool {
        self.deploy_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deploy_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.deploy_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deploy_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.deploy_time.is_none() {
            self.deploy_time.set_default();
        }
        self.deploy_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_deploy_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.deploy_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.protobuf.Timestamp undeploy_time = 3;


    pub fn get_undeploy_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.undeploy_time.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_undeploy_time(&mut self) {
        self.undeploy_time.clear();
    }

    pub fn has_undeploy_time(&self) -> bool {
        self.undeploy_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_undeploy_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.undeploy_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_undeploy_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.undeploy_time.is_none() {
            self.undeploy_time.set_default();
        }
        self.undeploy_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_undeploy_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.undeploy_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // string config = 4;


    pub fn get_config(&self) -> &str {
        &self.config
    }
    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::std::string::String) {
        self.config = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::std::string::String {
        &mut self.config
    }

    // Take field
    pub fn take_config(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.config, ::std::string::String::new())
    }

    // string address = 5;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // repeated string resource_uri = 6;


    pub fn get_resource_uri(&self) -> &[::std::string::String] {
        &self.resource_uri
    }
    pub fn clear_resource_uri(&mut self) {
        self.resource_uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_uri(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.resource_uri = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_uri(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resource_uri
    }

    // Take field
    pub fn take_resource_uri(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.resource_uri, ::protobuf::RepeatedField::new())
    }

    // .grafeas.v1beta1.deployment.Deployment.Platform platform = 7;


    pub fn get_platform(&self) -> Deployment_Platform {
        self.platform
    }
    pub fn clear_platform(&mut self) {
        self.platform = Deployment_Platform::PLATFORM_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: Deployment_Platform) {
        self.platform = v;
    }
}

impl ::protobuf::Message for Deployment {
    fn is_initialized(&self) -> bool {
        for v in &self.deploy_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.undeploy_time {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deploy_time)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.undeploy_time)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.config)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resource_uri)?;
                },
                7 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.platform, 7, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.user_email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_email);
        }
        if let Some(ref v) = self.deploy_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.undeploy_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.config.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.config);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.address);
        }
        for value in &self.resource_uri {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.platform != Deployment_Platform::PLATFORM_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(7, self.platform);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.user_email.is_empty() {
            os.write_string(1, &self.user_email)?;
        }
        if let Some(ref v) = self.deploy_time.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.undeploy_time.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.config.is_empty() {
            os.write_string(4, &self.config)?;
        }
        if !self.address.is_empty() {
            os.write_string(5, &self.address)?;
        }
        for v in &self.resource_uri {
            os.write_string(6, &v)?;
        };
        if self.platform != Deployment_Platform::PLATFORM_UNSPECIFIED {
            os.write_enum(7, self.platform.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Deployment {
        Deployment::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user_email",
                    |m: &Deployment| { &m.user_email },
                    |m: &mut Deployment| { &mut m.user_email },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "deploy_time",
                    |m: &Deployment| { &m.deploy_time },
                    |m: &mut Deployment| { &mut m.deploy_time },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "undeploy_time",
                    |m: &Deployment| { &m.undeploy_time },
                    |m: &mut Deployment| { &mut m.undeploy_time },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config",
                    |m: &Deployment| { &m.config },
                    |m: &mut Deployment| { &mut m.config },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &Deployment| { &m.address },
                    |m: &mut Deployment| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_uri",
                    |m: &Deployment| { &m.resource_uri },
                    |m: &mut Deployment| { &mut m.resource_uri },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Deployment_Platform>>(
                    "platform",
                    |m: &Deployment| { &m.platform },
                    |m: &mut Deployment| { &mut m.platform },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Deployment>(
                    "Deployment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Deployment {
        static mut instance: ::protobuf::lazy::Lazy<Deployment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Deployment,
        };
        unsafe {
            instance.get(Deployment::new)
        }
    }
}

impl ::protobuf::Clear for Deployment {
    fn clear(&mut self) {
        self.user_email.clear();
        self.deploy_time.clear();
        self.undeploy_time.clear();
        self.config.clear();
        self.address.clear();
        self.resource_uri.clear();
        self.platform = Deployment_Platform::PLATFORM_UNSPECIFIED;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Deployment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Deployment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Deployment_Platform {
    PLATFORM_UNSPECIFIED = 0,
    GKE = 1,
    FLEX = 2,
    CUSTOM = 3,
}

impl ::protobuf::ProtobufEnum for Deployment_Platform {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Deployment_Platform> {
        match value {
            0 => ::std::option::Option::Some(Deployment_Platform::PLATFORM_UNSPECIFIED),
            1 => ::std::option::Option::Some(Deployment_Platform::GKE),
            2 => ::std::option::Option::Some(Deployment_Platform::FLEX),
            3 => ::std::option::Option::Some(Deployment_Platform::CUSTOM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Deployment_Platform] = &[
            Deployment_Platform::PLATFORM_UNSPECIFIED,
            Deployment_Platform::GKE,
            Deployment_Platform::FLEX,
            Deployment_Platform::CUSTOM,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Deployment_Platform", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Deployment_Platform {
}

impl ::std::default::Default for Deployment_Platform {
    fn default() -> Self {
        Deployment_Platform::PLATFORM_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for Deployment_Platform {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eproto/v1beta1/deployment.proto\x12\x1agrafeas.v1beta1.deployment\
    \x1a\x1fgoogle/protobuf/timestamp.proto\"/\n\nDeployable\x12!\n\x0cresou\
    rce_uri\x18\x01\x20\x03(\tR\x0bresourceUri\"Q\n\x07Details\x12F\n\ndeplo\
    yment\x18\x01\x20\x01(\x0b2&.grafeas.v1beta1.deployment.DeploymentR\ndep\
    loyment\"\x90\x03\n\nDeployment\x12\x1d\n\nuser_email\x18\x01\x20\x01(\t\
    R\tuserEmail\x12;\n\x0bdeploy_time\x18\x02\x20\x01(\x0b2\x1a.google.prot\
    obuf.TimestampR\ndeployTime\x12?\n\rundeploy_time\x18\x03\x20\x01(\x0b2\
    \x1a.google.protobuf.TimestampR\x0cundeployTime\x12\x16\n\x06config\x18\
    \x04\x20\x01(\tR\x06config\x12\x18\n\x07address\x18\x05\x20\x01(\tR\x07a\
    ddress\x12!\n\x0cresource_uri\x18\x06\x20\x03(\tR\x0bresourceUri\x12K\n\
    \x08platform\x18\x07\x20\x01(\x0e2/.grafeas.v1beta1.deployment.Deploymen\
    t.PlatformR\x08platform\"C\n\x08Platform\x12\x18\n\x14PLATFORM_UNSPECIFI\
    ED\x10\0\x12\x07\n\x03GKE\x10\x01\x12\x08\n\x04FLEX\x10\x02\x12\n\n\x06C\
    USTOM\x10\x03Be\n\x1dio.grafeas.v1beta1.deploymentP\x01Z<github.com/graf\
    eas/grafeas/proto/v1beta1/deployment_go_proto\xa2\x02\x03GRAJ\x9c\x13\n\
    \x06\x12\x04\x0e\0I\x01\n\xda\x04\n\x01\x0c\x12\x03\x0e\0\x122\xcf\x04\
    \x20Copyright\x202018\x20The\x20Grafeas\x20Authors.\x20All\x20rights\x20\
    reserved.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Versi\
    on\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20\
    required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writi\
    ng,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20dis\
    tributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIE\
    S\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0#\n\x08\n\x01\x08\x12\x03\
    \x12\0S\n\t\n\x02\x08\x0b\x12\x03\x12\0S\n\x08\n\x01\x08\x12\x03\x13\0\"\
    \n\t\n\x02\x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\06\n\t\n\
    \x02\x08\x01\x12\x03\x14\06\n\x08\n\x01\x08\x12\x03\x15\0!\n\t\n\x02\x08\
    $\x12\x03\x15\0!\n\t\n\x02\x03\0\x12\x03\x17\0)\n?\n\x02\x04\0\x12\x04\
    \x1a\0\x1d\x01\x1a3\x20An\x20artifact\x20that\x20can\x20be\x20deployed\
    \x20in\x20some\x20runtime.\n\n\n\n\x03\x04\0\x01\x12\x03\x1a\x08\x12\nF\
    \n\x04\x04\0\x02\0\x12\x03\x1c\x02#\x1a9\x20Required.\x20Resource\x20URI\
    \x20for\x20the\x20artifact\x20being\x20deployed.\n\n\x0c\n\x05\x04\0\x02\
    \0\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1c\x0b\x11\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1c\x12\x1e\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x1c!\"\n1\n\x02\x04\x01\x12\x04\x20\0#\x01\x1a%\x20Details\
    \x20of\x20a\x20deployment\x20occurrence.\n\n\n\n\x03\x04\x01\x01\x12\x03\
    \x20\x08\x0f\n=\n\x04\x04\x01\x02\0\x12\x03\"\x02\x1c\x1a0\x20Required.\
    \x20Deployment\x20history\x20for\x20the\x20resource.\n\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\"\x02\x20\x11\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\"\
    \x02\x0c\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\"\r\x17\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\"\x1a\x1b\nN\n\x02\x04\x02\x12\x04&\0I\x01\x1aB\
    \x20The\x20period\x20during\x20which\x20some\x20deployable\x20was\x20act\
    ive\x20in\x20a\x20runtime.\n\n\n\n\x03\x04\x02\x01\x12\x03&\x08\x12\nC\n\
    \x04\x04\x02\x02\0\x12\x03(\x02\x18\x1a6\x20Identity\x20of\x20the\x20use\
    r\x20that\x20triggered\x20this\x20deployment.\n\n\r\n\x05\x04\x02\x02\0\
    \x04\x12\x04(\x02&\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03(\x02\x08\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03(\t\x13\n\x0c\n\x05\x04\x02\x02\0\
    \x03\x12\x03(\x16\x17\nF\n\x04\x04\x02\x02\x01\x12\x03+\x02,\x1a9\x20Req\
    uired.\x20Beginning\x20of\x20the\x20lifetime\x20of\x20this\x20deployment\
    .\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04+\x02(\x18\n\x0c\n\x05\x04\x02\
    \x02\x01\x06\x12\x03+\x02\x1b\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03+\
    \x1c'\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03+*+\n6\n\x04\x04\x02\x02\
    \x02\x12\x03.\x02.\x1a)\x20End\x20of\x20the\x20lifetime\x20of\x20this\
    \x20deployment.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04.\x02+,\n\x0c\n\
    \x05\x04\x02\x02\x02\x06\x12\x03.\x02\x1b\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03.\x1c)\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03.,-\n<\n\x04\
    \x04\x02\x02\x03\x12\x031\x02\x14\x1a/\x20Configuration\x20used\x20to\
    \x20create\x20this\x20deployment.\n\n\r\n\x05\x04\x02\x02\x03\x04\x12\
    \x041\x02..\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x031\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x03\x01\x12\x031\t\x0f\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x031\x12\x13\nF\n\x04\x04\x02\x02\x04\x12\x034\x02\x15\x1a9\x20Address\
    \x20of\x20the\x20runtime\x20element\x20hosting\x20this\x20deployment.\n\
    \n\r\n\x05\x04\x02\x02\x04\x04\x12\x044\x021\x14\n\x0c\n\x05\x04\x02\x02\
    \x04\x05\x12\x034\x02\x08\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x034\t\x10\
    \n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x034\x13\x14\n}\n\x04\x04\x02\x02\
    \x05\x12\x038\x02#\x1ap\x20Output\x20only.\x20Resource\x20URI\x20for\x20\
    the\x20artifact\x20being\x20deployed\x20taken\x20from\n\x20the\x20deploy\
    able\x20field\x20with\x20the\x20same\x20name.\n\n\x0c\n\x05\x04\x02\x02\
    \x05\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x038\x0b\x11\
    \n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x038\x12\x1e\n\x0c\n\x05\x04\x02\
    \x02\x05\x03\x12\x038!\"\n#\n\x04\x04\x02\x04\0\x12\x04;\x02D\x03\x1a\
    \x15\x20Types\x20of\x20platforms.\n\n\x0c\n\x05\x04\x02\x04\0\x01\x12\
    \x03;\x07\x0f\n\x19\n\x06\x04\x02\x04\0\x02\0\x12\x03=\x04\x1d\x1a\n\x20\
    Unknown.\n\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03=\x04\x18\n\x0e\n\
    \x07\x04\x02\x04\0\x02\0\x02\x12\x03=\x1b\x1c\n)\n\x06\x04\x02\x04\0\x02\
    \x01\x12\x03?\x04\x0c\x1a\x1a\x20Google\x20Container\x20Engine.\n\n\x0e\
    \n\x07\x04\x02\x04\0\x02\x01\x01\x12\x03?\x04\x07\n\x0e\n\x07\x04\x02\
    \x04\0\x02\x01\x02\x12\x03?\n\x0b\n9\n\x06\x04\x02\x04\0\x02\x02\x12\x03\
    A\x04\r\x1a*\x20Google\x20App\x20Engine:\x20Flexible\x20Environment.\n\n\
    \x0e\n\x07\x04\x02\x04\0\x02\x02\x01\x12\x03A\x04\x08\n\x0e\n\x07\x04\
    \x02\x04\0\x02\x02\x02\x12\x03A\x0b\x0c\n.\n\x06\x04\x02\x04\0\x02\x03\
    \x12\x03C\x04\x0f\x1a\x1f\x20Custom\x20user-defined\x20platform.\n\n\x0e\
    \n\x07\x04\x02\x04\0\x02\x03\x01\x12\x03C\x04\n\n\x0e\n\x07\x04\x02\x04\
    \0\x02\x03\x02\x12\x03C\r\x0e\n0\n\x04\x04\x02\x02\x06\x12\x03F\x02\x18\
    \x1a#\x20Platform\x20hosting\x20this\x20deployment.\n\n\r\n\x05\x04\x02\
    \x02\x06\x04\x12\x04F\x02D\x03\n\x0c\n\x05\x04\x02\x02\x06\x06\x12\x03F\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x06\x01\x12\x03F\x0b\x13\n\x0c\n\x05\x04\
    \x02\x02\x06\x03\x12\x03F\x16\x17b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
