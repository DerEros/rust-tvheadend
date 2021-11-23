use anyhow::Result;
use std::io::Write;

#[derive(Copy, Clone, Debug)]
enum FieldType {
    Map = 1,
    S64,
    Str,
    Bin,
    List,
}

pub struct Field<T> {
    name: &'static str,
    value: T,
}

pub trait FieldT {
    fn get_name(&self) -> &'static str;
    fn get_name_length(&self) -> u8;
}

impl<T> Field<T> {
    pub fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

pub fn field<T>(name: &'static str, value: T) -> Field<T> {
    Field::new(name, value)
}

impl<T> FieldT for Field<T> {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_name_length(&self) -> u8 {
        let len = self.name.len();
        if len > 255 {
            panic!("Field names longer than 255 characters are not supported")
        }
        len as u8
    }
}

pub fn serialize<T, U>(field: &U, out: &mut dyn Write) -> Result<()>
where
    U: FieldT + SerializableField<T>,
{
    out.write_all(&[field.get_type_id()])?; // Type
    out.write_all(&[field.get_name_length()])?; // Name length
    out.write_all(&field.get_data_length().to_be_bytes())?; // Data length
    out.write_all(field.get_name().as_bytes())?; // Name
    field.serialize_value(out)?; // Payload

    Ok(())
}

pub fn get_total_field_size<T, U>(field: &U) -> u32
where
    U: FieldT + SerializableField<T>,
{
    let len: u32 = 1 +              // Type
        1 +                         // Name length
        4 +                         // Data length
        field.get_name_length() as u32 +
        field.get_data_length();
    len
}

pub trait SerializableField<T> {
    fn get_type_id(&self) -> u8;
    fn get_data_length(&self) -> u32;
    fn serialize_value(&self, out: &mut dyn Write) -> Result<()>;
}

impl SerializableField<&'static str> for Field<&'static str> {
    fn get_type_id(&self) -> u8 {
        FieldType::Str as u8
    }

    fn get_data_length(&self) -> u32 {
        self.value.len() as u32
    }

    fn serialize_value(&self, out: &mut dyn Write) -> Result<()> {
        out.write_all(self.value.as_bytes())?;
        Ok(())
    }
}

impl SerializableField<u32> for Field<u32> {
    fn get_type_id(&self) -> u8 {
        FieldType::S64 as u8
    }

    fn get_data_length(&self) -> u32 {
        let mut len = 0;
        let mut value = self.value;
        while value != 0 {
            len += 1;
            value >>= 8;
        }

        len
    }

    fn serialize_value(&self, out: &mut dyn Write) -> Result<()> {
        let mut value = self.value;
        while value != 0 {
            out.write_all(&[(self.value & 0xFF) as u8])?;
            value >>= 8;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn test_name_length() {
        let field = Field {
            name: "MyName",
            value: "FooBar",
        };

        assert_eq!(6, field.get_name_length());
    }

    #[test]
    fn test_s64_short_length() {
        let field = Field {
            name: "Foo",
            value: 1 as u32,
        };

        assert_eq!(1, field.get_data_length());
    }

    #[test]
    fn test_s64_long_length() {
        let field = Field {
            name: "Foo",
            value: 2_u32.pow(31) as u32,
        };

        assert_eq!(4, field.get_data_length());
    }

    #[test]
    fn test_str_field_type() {
        let field = Field {
            name: "Foo",
            value: "Bar",
        };

        assert_eq!(3, field.get_type_id());
    }

    #[test]
    fn test_u32_field_type() {
        let field = Field {
            name: "foo",
            value: 123 as u32,
        };

        assert_eq!(2, field.get_type_id());
    }

    #[test]
    fn test_serialize_field() {
        let field = Field {
            name: "FooBar",
            value: 42 as u32,
        };

        let expectation = vec![2, 6, 1, 0, 0, 0, 70, 111, 111, 66, 97, 114, 42];
        let mut result: Vec<u8> = vec![];
        serialize(&field, &mut result);
        assert_eq!(expectation, result);
    }
}
