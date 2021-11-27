use super::intermediate::Fields;
use crate::protocol::intermediate::{Field, FieldData};
use bytes::{BufMut, Bytes, BytesMut};
use log::*;

pub trait ToBytes {
    fn to_bytes(&self) -> Bytes;
}

impl ToBytes for Fields {
    fn to_bytes(&self) -> Bytes {
        let total_length: usize = self.iter().map(|field| field.get_field_length()).sum();
        trace!("Total of {} bytes", total_length);

        let mut buffer = BytesMut::with_capacity(total_length);
        buffer.put_u32(total_length as u32);
        for field in self {
            serialize_field(field, &mut buffer);
        }

        buffer.freeze()
    }
}

fn serialize_field(field: &Field, buffer: &mut BytesMut) {
    let type_id = get_type_id(field.get_data());
    let name_length = field.get_name_length();
    let data_length = field.get_data_length();
    let name = field.get_name();

    buffer.put_u8(type_id);
    buffer.put_u8(name_length as u8);
    buffer.put_u32(data_length as u32);
    buffer.put(name.as_bytes());
    serialize_data(field.get_data(), buffer);
}

fn get_type_id(data: &FieldData) -> u8 {
    match data {
        FieldData::Map(_) => 1,
        FieldData::S64(_) => 2,
        FieldData::Str(_) => 3,
        FieldData::Bin(_) => 4,
        FieldData::List(_) => 5,
    }
}

fn i64_2_wire_format(i: i64) -> BytesMut {
    let mut i_mut = i;
    let mut res = BytesMut::new();
    while i_mut != 0 {
        res.put_u8((i_mut & 0xFF) as u8);
        i_mut >>= 8;
    }

    res
}

fn serialize_data(data: &FieldData, buffer: &mut BytesMut) {
    match data {
        FieldData::Map(_) => todo!(),
        FieldData::S64(i) => serialize_s64(i, buffer),
        FieldData::Str(s) => serialize_str(s, buffer),
        FieldData::Bin(b) => serialize_bin(b, buffer),
        FieldData::List(_) => todo!(),
    }
}

fn serialize_s64(i: &i64, buffer: &mut BytesMut) {
    buffer.put(i64_2_wire_format(*i));
}

fn serialize_str(s: &str, buffer: &mut BytesMut) {
    buffer.put(s.as_bytes());
}

fn serialize_bin(b: &Bytes, buffer: &mut BytesMut) {
    buffer.put(b.clone());
}
