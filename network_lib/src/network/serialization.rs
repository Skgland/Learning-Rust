use self::ReadWrite::*;
use std::prelude::v1::*;
use std::result::Result::Ok;
use std::io::prelude::*;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use crate::macros::Serializable;

///
/// an enum that determines if the serialize method from the Serialize trait serializes or deserialize
///

pub enum ReadWrite<'a> {
    READ { reader: &'a mut dyn Read },
    WRITE { writer: &'a mut dyn Write },
}

impl<'a> ReadWrite<'a> {
    fn flush(&mut self) -> Result<()> {
        match self {
            WRITE { writer } => writer.flush(),
            READ { .. } => Ok(())
        }
    }
}

/// a test struct for testing the procedural derive module for the Serialize trait
#[derive(Serializable)]
pub struct Test {
    pub name: String,
    pub age: i32,
}

///
/// the Serializable trait for (de-)serializing objects
///
///  ***NOTE:*** Intentionally not implemented for isize and usize as they are platform dependent
///
pub trait Serializable {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()>;
}

///
/// module containing a macro for implementing serializable for primitive integers excluding u8
///
mod serializable_integer_impl {
    use super::*;

    macro_rules! serialize_impl {
        ($t:ty) => {
            impl Serializable for $t {
                fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {

                    //create buffer from primitive
                    let mut buf =  self.to_le_bytes();

                    buf.serialize(direction)?;

                    *self = <$t>::from_le_bytes(buf);

                    Ok(())
                }
            }
        }
    }

    serialize_impl!(i8);
    serialize_impl!(i16);
    serialize_impl!(u16);
    serialize_impl!(u32);
    serialize_impl!(i32);
    serialize_impl!(i64);
    serialize_impl!(u64);
    serialize_impl!(u128);
    serialize_impl!(i128);
}


impl Serializable for u8 {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        match direction {
            READ { reader } => {
                let buf = &mut [0];
                reader.read_exact(buf)?;
                *self = buf[0];
                Ok(())
            }
            WRITE { writer } => {
                writer.write_all(&[*self])?;
                Ok(())
            }
        }
    }
}

impl Serializable for bool {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut tmp: u8 = 0;

        if let WRITE { .. } = direction {
            tmp = if *self { 1 } else { 0 };
        }

        tmp.serialize(direction)?;

        if let READ { .. } = direction {
            *self = tmp != 0;
        }

        Ok(())
    }
}

mod impl_serializable_vec {
    use super::*;


    impl<T> Serializable for Vec<T> where T: Serializable + Default {
        fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
            let mut length: u32 = 0;

            if let WRITE { .. } = direction {
                length = self.len() as u32;
            }

            length.serialize(direction)?;

            if let READ { .. } = direction {
                self.resize_with(length as usize, Default::default);
            }

            self.as_mut_slice().serialize(direction)?;
            Ok(())
        }
    }
}

mod impl_serializable_slice {
    use super::*;

    impl<T> Serializable for [T] where T: Serializable {
        fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
            for item in self {
                item.serialize(direction)?;
            }

            Ok(())
        }
    }
}

impl Serializable for String {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut vec: Vec<u8> = Vec::new();
        if let WRITE { .. } = direction {
            vec = Vec::from(self.as_bytes());
        }

        vec.serialize(direction)?;

        if let READ { .. } = direction {
            let tmp = self;

            String::from_utf8(vec)
                .map(|string| tmp.clone_from(&string))
                .map_err(|e| e.utf8_error())
                .map_err(|utf8| Error::new(ErrorKind::InvalidData, Box::new(utf8)))
        } else {
            Ok(())
        }
    }
}

impl<T> Serializable for Option<T> where T: Serializable + Default {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        if let WRITE { .. } = direction {
            match self {
                Some(obj) => {
                    true.serialize(direction)?;
                    obj.serialize(direction)?;
                }
                None => false.serialize(direction)?
            }
        } else {
            let mut is_some = false;
            is_some.serialize(direction)?;

            if is_some {
                let mut obj: T = T::default();
                obj.serialize(direction)?;
                *self = Some(obj);
            } else {
                *self = None;
            }
        }

        Ok(())
    }
}
