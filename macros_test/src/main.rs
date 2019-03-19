
fn main(){}

#[cfg(test)]
mod tests {
    use chat_base::prelude::v1::*;
    use std::collections::VecDeque;
    use std::io::*;
    use macros::*;

    struct TestWR<'a> {
        queue: &'a mut VecDeque<u8>
    }

    impl Write for TestWR<'_> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            for item in buf {
                self.queue.push_back(*item);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {Ok(())}
    }

    impl Read for TestWR<'_> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut read: usize = 0;
            for item in buf {
                if let Some(value) = self.queue.pop_front() {
                    *item = value;
                    read += 1;
                } else {
                    break;
                }
            }
            Ok(read)
        }
    }

    #[derive(Serializable)]
    struct AStruct {
        name: String,
        age: i32,
    }

    #[test]
    fn macro_test() -> Result<()> {

        let mut from = AStruct { name:  "Correct".to_string(), age: 42 };
        let mut to = AStruct { name: "Incorrect".to_string(), age: -42 };
        let mut queue:VecDeque<u8> = VecDeque::new();
        let mut stream = TestWR{queue:&mut queue};

        let write = &mut ReadWrite::WRITE { writer: &mut stream };

        from.serialize(write)?;

        let read = & mut ReadWrite::READ { reader: &mut stream };

        to.serialize(read)?;

        assert_eq!(to.name ,from.name);

        assert_eq!(to.age ,from.age);

        Ok(())
    }

}
