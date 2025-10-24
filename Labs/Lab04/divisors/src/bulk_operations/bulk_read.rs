use std::{
    io::{self, ErrorKind, Read},
    net::TcpStream,
};

pub fn bulk_read(stream: &mut TcpStream, size: usize) -> io::Result<Vec<u8>> {
    if size == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0u8; size];
    let mut read_total = 0usize;

    while read_total < size {
        match stream.read(&mut buffer[read_total..]) {
            Ok(0) => {
                buffer.truncate(read_total);
                return Ok(buffer);
            }
            Ok(n) => {
                read_total += n;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(buffer)
}
