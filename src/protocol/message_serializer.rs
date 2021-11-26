use anyhow::Result;
use log::*;
use std::fmt::Error;

use crate::protocol::field_serializer::FieldSerializer;
use crate::protocol::intermediate::Field;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};
use serde::Serialize;

use super::intermediate::Fields;

pub struct MessageSerializer {
    fields: Vec<Field>,
}

impl MessageSerializer {
    pub fn new<T: AsRef<str>>(method: T) -> Self {
        Self {
            fields: vec![Field::from_str("method".into(), method)],
        }
    }
}

impl Serializer for MessageSerializer {
    type Ok = Fields;
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[allow(unused_variables)]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl SerializeSeq for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
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

impl SerializeTuple for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
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

impl SerializeTupleStruct for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
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

impl SerializeTupleVariant for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
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

impl SerializeMap for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
    fn serialize_key<T: ?Sized>(
        &mut self,
        key: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    #[allow(unused_variables)]
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

impl SerializeStruct for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    #[allow(unused_variables)]
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

impl SerializeStructVariant for MessageSerializer {
    type Ok = Fields;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::rust_2015::Result<(), Self::Error>
    where
        T: Serialize,
    {
        trace!("Serialize field '{}'", key);
        let field = value.serialize(FieldSerializer::new(key))?;
        self.fields.push(field);

        Ok(())
    }

    fn end(self) -> std::prelude::rust_2015::Result<Self::Ok, Self::Error> {
        Ok(self.fields)
    }
}