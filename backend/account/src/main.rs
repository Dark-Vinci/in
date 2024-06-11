use account::Meme;

// #[warn(dead_code)]

fn main() {
    #[warn(dead_code)]
    let _m = Meme;
    println!("Hello, world!");

    use std::io::{self, Write};

    struct MyWriter {
        data: Vec<u8>,
    }

    impl MyWriter {
        fn new() -> Self {
            MyWriter { data: Vec::new() }
        }
    }

    // implement the Write trait for MyWriter
    impl Write for MyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut a = MyWriter::new();

    write!(a, "this would be written to MyStruct").unwrap();

}

