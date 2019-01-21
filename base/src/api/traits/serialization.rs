use self::ReadWrite::*;
use crate::macros::*;
use std::prelude::v1::*;
use std::result::Result::Ok;
use std::io::prelude::*;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

pub enum ReadWrite<'a> {
    READ { reader: &'a mut Read },
    WRITE { writer: &'a mut Write },
}


#[derive(Serializeable)]
pub struct Test {
    pub name: String,
    pub age: i32,
}

pub trait Serializeable {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()>;
}

mod serializeable_primitive_impl {
    use super::*;

    macro_rules! serialize_impl {
    ($t:ty, $c:expr) => {
        impl Serializeable for $t {
            fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {

                //assert!($c == $t) //TODO assert c matches t's size
                let buf = &mut [0u8; $c];


                // rust 1.32.0 will have stable to_le_bytes / from_le_bytes method

                if let WRITE { .. } = direction {
                    let mut cp = *self;

                    for item in buf.iter_mut() {
                        *item = cp as u8;
                        cp >>= 8;
                    }
                }

                buf.serialize(direction)?;

                if let READ { .. } = direction {
                    *self = 0;
                    for item in buf.iter().rev() {
                        *self <<= 8;
                        *self |= <$t>::from(*item);
                    }
                }

                Ok(())
            }
        }
    }
}

    //don't do u8 as this implementation works by butting the bytes into a slice than serializing that slice which will in turn serialize every element
//serialize_impl!(i8,1);
    serialize_impl! {u16,2}
    serialize_impl! {i16,2}
    serialize_impl! {u32,4}
    serialize_impl! {i32,4}
    serialize_impl! {u64,8}
    serialize_impl! {i64,8}
}

impl Serializeable for u8 {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let buf = &mut [0u8; 1];
// rust 1.32.0 will have stable to_le_bytes method

        if let WRITE { writer } = direction {
            buf[0] = *self;
            writer.write_all(buf)?;
        }
        if let READ { reader } = direction {
            reader.read_exact(buf)?;
            *self = buf[0];
        }

        Ok(())
    }
}

impl<T> Serializeable for Vec<T> where T: Serializeable + Default {

    default fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut length: u32 = 0;

        if let WRITE { .. } = direction {
            length = self.len() as u32;
        }

        length.serialize(direction)?;

        if let READ { .. } = direction {
            self.resize_with(length as usize,T::default);
        }

        self.as_mut_slice().serialize(direction)?;
        Ok(())
    }
}

impl<T> Serializeable for Vec<T> where T: Serializeable + Clone + Default {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut length: u32 = 0;

        if let WRITE { .. } = direction {
            length = self.len() as u32;
        }

        length.serialize(direction)?;

        if let READ { .. } = direction {
            self.resize(length as usize, T::default());
        }

        self.as_mut_slice().serialize(direction)?;
        Ok(())
    }
}


impl<T> Serializeable for [T] where T: Serializeable {
    default fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        for item in self {
            item.serialize(direction)?;
        }

        Ok(())
    }
}

impl Serializeable for [u8] {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        match direction {
            WRITE { writer } => writer.write_all(self),
            READ { reader } => reader.read_exact(self)
        }
    }
}

impl Serializeable for &'_ mut str {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        self.to_string().serialize(direction)
    }
}

impl Serializeable for String {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut vec: Vec<u8> = Vec::new();
        if let WRITE { .. } = direction {
            vec = Vec::from(self.as_bytes());
        }

        vec.serialize(direction)?;

        if let READ { .. } = direction {
            let result = &String::from_utf8(vec);
            match result {
                Ok(string) => {
                    self.clone_from(string);
                    Ok(())
                }
                Err(e) => {
                    Err(Error::new(ErrorKind::InvalidData, Box::new(e.utf8_error())))
                }
            }
        } else {
            Ok(())
        }
    }
}
