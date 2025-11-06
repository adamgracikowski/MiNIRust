use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use divisors::{IP_ADDRESS, PORT};

fn main() -> io::Result<()> {
    let path = {
        let args = env::args().skip(1).collect::<Vec<_>>();
        if args.len() != 1 {
            eprintln!("Usage: client <path>");
            eprintln!("Example: cargo run -- \"/path/to/some/dir\"");
            return Ok(());
        }
        args.join("")
    };

    let path_bytes = path.as_bytes();
    if path_bytes.len() > u32::MAX as usize {
        eprintln!("Path too long");
        return Ok(());
    }

    let mut stream = TcpStream::connect((IP_ADDRESS, PORT))?;

    let len_be = (path_bytes.len() as u32).to_be_bytes();

    stream.write_all(&len_be)?;
    stream.write_all(path_bytes)?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let response = String::from_utf8_lossy(&response);

    println!("{response}");

    Ok(())
}
