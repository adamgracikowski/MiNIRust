use std::{
    io::{self, ErrorKind, Write},
    net::TcpStream,
};

pub fn bulk_write(stream: &mut TcpStream, mut buffer: &[u8]) -> io::Result<()> {
    while !buffer.is_empty() {
        match stream.write(buffer) {
            Ok(0) => {
                return Err(io::Error::new(
                    ErrorKind::WriteZero,
                    "Failed to write to stream",
                ));
            }
            Ok(n) => {
                buffer = &buffer[n..];
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
