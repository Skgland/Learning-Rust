use self::ReadWrite::*;
use ::macros::*;
use std::prelude::v1::*;
use std::result::Result::Ok;
use std::io::prelude::*;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

///
/// an enum that determines if the serialize method from the Serialize trait serializes or deserialize
///

pub enum ReadWrite<'a> {
    READ { reader: &'a mut Read },
    WRITE { writer: &'a mut Write },
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

    serialize_impl!(u8);
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

impl Serializable for bool {
    fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
        let mut tmp = 0;

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
        default fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
            let mut length: u32 = 0;

            if let WRITE { .. } = direction {
                length = self.len() as u32;
            }

            length.serialize(direction)?;

            if let READ { .. } = direction {
                self.resize_with(length as usize, T::default);
            }

            self.as_mut_slice().serialize(direction)?;
            Ok(())
        }
    }

    impl<T> Serializable for Vec<T> where T: Serializable + Clone + Default {
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
}

mod impl_serializable_slice {
    use super::*;

    impl<T> Serializable for [T] where T: Serializable {
        default fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
            for item in self {
                item.serialize(direction)?;
            }

            Ok(())
        }
    }

    impl Serializable for [u8] {
        fn serialize(&mut self, direction: &mut ReadWrite) -> Result<()> {
            match direction {
                WRITE { writer } => writer.write_all(self),
                READ { reader } => reader.read_exact(self)
            }
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
