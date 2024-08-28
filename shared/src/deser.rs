use core::fmt::{Display, Formatter};
use core::marker::PhantomData;

use funty::{AtMost32, Unsigned};
use serde::ser;

type Result<T> = core::result::Result<T, Error>;

pub trait Buffer {
    type Output;

    fn copy_from_slice(&mut self, data: &[u8]) -> Result<()> {
        return data.iter().try_for_each(|d| self.push(*d));
    }

    fn push(&mut self, data: u8) -> Result<()>;
    fn finalize(self) -> Result<Self::Output>;
}

pub struct Slice<'a> {
    pub output: &'a mut [u8],
    pub pos: usize,
}

impl<'a> Slice<'a> {
    pub fn new(output: &'a mut [u8]) -> Self {
        return Self { output, pos: 0 };
    }
}

impl<'a> Buffer for Slice<'a> {
    type Output = &'a mut [u8];

    fn push(&mut self, data: u8) -> Result<()> {
        if self.pos == self.output.len() {
            return Err(Error::NoSpaceLeft);
        } else {
            self.output[self.pos] = data;
            self.pos += 1;
            return Ok(());
        }
    }

    fn finalize(self) -> Result<Self::Output> {
        return Ok(&mut self.output[..self.pos]);
    }

    fn copy_from_slice(&mut self, data: &[u8]) -> Result<()> {
        if (self.output.len() - self.pos) < data.len() {
            return Err(Error::NoSpaceLeft);
        }
        let end = self.pos + data.len();
        self.output[self.pos..end].copy_from_slice(&data);
        self.pos = end;
        return Ok(());
    }
}

pub struct Serializer<OUT, LenType> {
    pub output: OUT,
    _lt: PhantomData<LenType>,
}

impl<OUT, LenType> Serializer<OUT, LenType>
where
    OUT: Buffer,
    LenType: Unsigned + AtMost32 + From<usize>,
    usize: From<LenType>,
{
    fn push_usize_len(&mut self, value: usize) -> Result<()> {
        let len_max: usize = LenType::MAX.into();
        if value > len_max {
            return Err(Error::LenTooLarge);
        }
        let value: LenType = value.into();
        self.output.copy_from_slice(&value.to_le_bytes())?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Custom,
    NoSpaceLeft,
    NoDisplay,
    LenTooLarge,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                Custom => "Custom error",
                NoSpaceLeft => "Buffer is full",
                NoDisplay => "No collect_str available",
                LenTooLarge => "Lentgth too large",
            }
        )
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom
    }
}

impl serde::ser::StdError for Error {}

impl<'a, OUT, LenType> ser::Serializer for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        return self.output.copy_from_slice(&v.to_le_bytes());
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buf = [0u8; 4];
        let strsl = v.encode_utf8(&mut buf);
        return self.serialize_str(strsl);
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        if v.len() > u32::MAX as usize {
            return Err(Error::NoSpaceLeft);
        }
        self.push_usize_len(v.len())?;
        self.serialize_bytes(&v.as_bytes())?;
        return Ok(());
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        if v.len() > u32::MAX as usize {
            return Err(Error::NoSpaceLeft);
        }
        self.output
            .copy_from_slice(&(v.len() as u32).to_le_bytes())?;
        return self.output.copy_from_slice(v);
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        return self.serialize_u8(0);
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_u8(1)?;
        return value.serialize(self);
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        return self.serialize_u32(variant_index);
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        return value.serialize(self);
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_u32(variant_index)?;
        return value.serialize(self);
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len = len.ok_or(Error::NoSpaceLeft)?;
        let len = len as u32;
        self.output.copy_from_slice(&len.to_le_bytes())?;
        return Ok(self);
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        return Ok(self);
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        return Ok(self);
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.copy_from_slice(&variant_index.to_le_bytes())?;
        return Ok(self);
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let len = len.ok_or(Error::NoSpaceLeft)?;
        let len = len as u32;
        self.output.copy_from_slice(&len.to_le_bytes())?;
        return Ok(self);
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        return Ok(self);
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.copy_from_slice(&variant_index.to_le_bytes())?;
        return Ok(self);
    }

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        return Err(Error::NoDisplay);
    }
}

impl<'a, OUT, LenType> ser::SerializeSeq for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeTuple for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeTupleStruct for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeTupleVariant for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeMap for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeStruct for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, OUT, LenType> ser::SerializeStructVariant for &'a mut Serializer<OUT, LenType>
where
    OUT: Buffer,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_with_space_left() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 20];
        let mut out = Slice::new(&mut dst);
        let copied = out.copy_from_slice(&src);
        assert!(copied == Ok(()));
    }
}
