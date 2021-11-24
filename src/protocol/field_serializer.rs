use anyhow::Result;
use log::*;
use std::fmt::Error;

use crate::protocol::intermediate::{Field, FieldData};
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serializer;

pub struct FieldSerializer {
    name: String,
}

impl FieldSerializer {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl Serializer for FieldSerializer {
    type Ok = Field;
    type Error = std::fmt::Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        trace!("Serializing u32 '{}'", v);
        Ok(Field::from_u32(self.name, v))
    }

    fn serialize_u64(self, v: u64) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        trace!("Serializing str '{}'", v);
        Ok(Field::from_str(self.name, v))
    }

    fn serialize_bytes(self, v: &[u8]) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(
        self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::prelude::rust_2015::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> std::prelude::rust_2015::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::rust_2015::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::rust_2015::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::prelude::rust_2015::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::rust_2015::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::rust_2015::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl SerializeSeq for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTuple for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTupleStruct for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTupleVariant for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeMap for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_key<T: ?Sized>(
        &mut self,
        key: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStruct for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStructVariant for FieldSerializer {
    type Ok = Field;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
