use bytes::Bytes;
use std::collections::HashMap;
use std::ops::Shr;

#[derive(Debug)]
pub enum FieldData {
    Map(HashMap<String, Field>),
    S64(i64),
    Str(String),
    Bin(Bytes),
    List(Vec<Field>),
}

#[derive(Debug)]
pub struct Field {
    name: String,
    length: usize,
    data: FieldData,
}

pub type Fields = Vec<Field>;

impl Field {
    pub fn from_u32(name: String, value: u32) -> Self {
        Field {
            name,
            length: Self::calc_number_length(value as i64),
            data: FieldData::S64(value as i64),
        }
    }

    pub fn from_str<T: AsRef<str>>(name: String, value: T) -> Self {
        Field {
            name,
            length: value.as_ref().len(),
            data: FieldData::Str(value.as_ref().to_string()),
        }
    }

    fn calc_number_length(value: i64) -> usize {
        let mut v = value;
        let mut len = 0;
        while v != 0 {
            v >>= 8;
            len += 1;
        }
        len
    }

    pub fn get_name_length(&self) -> usize {
        self.name.len()
    }

    pub fn get_data_length(&self) -> usize {
        self.length
    }

    pub fn get_field_length(&self) -> usize {
        // 1 for field type, 1 for name length, 4 for data length
        self.get_name_length() + self.get_data_length() + 1 + 1 + 4
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_data(&self) -> &FieldData {
        &self.data
    }
}
