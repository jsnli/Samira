#![allow(dead_code)]

use std::collections::HashMap;

pub type VdfMap = HashMap<String, VdfValue>;

#[derive(Debug, Clone)]
pub enum VdfValue {
    Nested(VdfMap),
    Str(String),
    Int(i32),
    Float(f32),
    UInt64(u64),
}

impl VdfValue {
    pub fn as_map(&self) -> Option<&VdfMap> {
        if let VdfValue::Nested(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let VdfValue::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let VdfValue::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let VdfValue::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum VdfError {
    UnexpectedEof,
    UnknownType(u8),
    InvalidUtf8,
    BadSlice,
}

impl std::fmt::Display for VdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdfError::UnexpectedEof => write!(f, "Unexpected end of file"),
            VdfError::UnknownType(b) => write!(f, "Unknown type byte: 0x{:02X}", b),
            VdfError::InvalidUtf8 => write!(f, "Invalid UTF-8 in key or string value"),
            VdfError::BadSlice => write!(f, "Could not read fixed-width value (not enough bytes)"),
        }
    }
}

pub struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read_byte(&mut self) -> Result<u8, VdfError> {
        let b = self
            .bytes
            .get(self.pos)
            .copied()
            .ok_or(VdfError::UnexpectedEof)?;
        self.pos += 1;
        Ok(b)
    }

    fn read_cstring(&mut self) -> Result<String, VdfError> {
        let start = self.pos;
        loop {
            if self.pos >= self.bytes.len() {
                return Err(VdfError::UnexpectedEof);
            }
            if self.bytes[self.pos] == 0 {
                let s = std::str::from_utf8(&self.bytes[start..self.pos])
                    .map_err(|_| VdfError::InvalidUtf8)?
                    .to_owned();
                self.pos += 1;
                return Ok(s);
            }
            self.pos += 1;
        }
    }

    fn read_i32(&mut self) -> Result<i32, VdfError> {
        let slice = self
            .bytes
            .get(self.pos..self.pos + 4)
            .ok_or(VdfError::BadSlice)?;
        let v = i32::from_le_bytes(slice.try_into().map_err(|_| VdfError::BadSlice)?);
        self.pos += 4;
        Ok(v)
    }

    fn read_f32(&mut self) -> Result<f32, VdfError> {
        let slice = self
            .bytes
            .get(self.pos..self.pos + 4)
            .ok_or(VdfError::BadSlice)?;
        let v = f32::from_le_bytes(slice.try_into().map_err(|_| VdfError::BadSlice)?);
        self.pos += 4;
        Ok(v)
    }

    fn read_u64(&mut self) -> Result<u64, VdfError> {
        let slice = self
            .bytes
            .get(self.pos..self.pos + 8)
            .ok_or(VdfError::BadSlice)?;
        let v = u64::from_le_bytes(slice.try_into().map_err(|_| VdfError::BadSlice)?);
        self.pos += 8;
        Ok(v)
    }

    pub fn parse_object(&mut self) -> Result<VdfMap, VdfError> {
        let mut map = VdfMap::new();

        loop {
            let type_byte = self.read_byte()?;

            if type_byte == 0x08 {
                break;
            }

            let key = self.read_cstring()?;

            let value = match type_byte {
                0x00 => VdfValue::Nested(self.parse_object()?),
                0x01 => VdfValue::Str(self.read_cstring()?),
                0x02 => VdfValue::Int(self.read_i32()?),
                0x03 => VdfValue::Float(self.read_f32()?),
                0x07 => VdfValue::UInt64(self.read_u64()?),
                other => return Err(VdfError::UnknownType(other)),
            };

            map.insert(key, value);
        }

        Ok(map)
    }
}
