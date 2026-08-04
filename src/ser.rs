use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{Serialize, ser};

use crate::error::Error;

pub struct Serializer {
    pub output: String,
}

impl<'ser> ser::Serializer for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let v: String = Base64UrlUnpadded::encode_string(v);

        self.output += v.as_str();

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    /// If this is already a string then just add it to the output.
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += v;

        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeBool)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeChar)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeFloat)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeFloat)
    }
    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedTypeInteger)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedEmptyType)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnexpectedEmptyType)
    }
}

impl<'ser> ser::SerializeSeq for &'ser mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('.') {
            self.output += ".";
        }

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'ser> ser::SerializeMap for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnableToDetermineElementOrder)
    }
}

impl<'ser> ser::SerializeStruct for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnableToDetermineElementOrder)
    }
}

impl<'ser> ser::SerializeStructVariant for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnableToDetermineElementOrder)
    }

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnableToDetermineElementOrder)
    }
}

impl<'ser> ser::SerializeTuple for &'ser mut Serializer {
    type Ok = ();

    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('.') {
            self.output += ".";
        }

        value.serialize(&mut **self)
    }
}

impl<'ser> ser::SerializeTupleStruct for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('.') {
            self.output += ".";
        }

        value.serialize(&mut **self)
    }
}

impl<'ser> ser::SerializeTupleVariant for &'ser mut Serializer {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('.') {
            self.output += ".";
        }

        value.serialize(&mut **self)
    }
}
