use std::{
    fs, io,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    str::FromStr,
};

use divisors::{IP_ADDRESS, PORT, bulk_read, bulk_write};

pub struct DirectoryServer {
    pub address: String,
    pub port: u16,
}

impl DirectoryServer {
    fn new(address: &str, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let bind_address = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(bind_address.clone())?;

        println!("Server listening on {bind_address}");

        for connection_stream in listener.incoming() {
            match connection_stream {
                Ok(stream) => match self.handle_client(stream) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("An error occurred while handling the client: {e}");
                    }
                },
                Err(e) => {
                    eprintln!("Incoming connection failed: {e}");
                }
            }
        }

        Ok(())
    }

    fn list_dir(&self, pathbuf: &PathBuf) -> Option<String> {
        match fs::read_dir(pathbuf) {
            Ok(entries) => {
                let mut listing = String::new();
                for dir_entry in entries {
                    match dir_entry {
                        Ok(entry) => {
                            let metadata = match entry.metadata() {
                                Ok(metadata) => metadata,
                                Err(e) => {
                                    eprintln!(
                                        "Error getting metadata for entry in {pathbuf:?}: {e}"
                                    );
                                    return None;
                                }
                            };

                            if !metadata.is_file() {
                                continue;
                            }

                            let file_name = entry.file_name();
                            let name = file_name.to_string_lossy();
                            listing.push_str(&name);
                            listing.push('\n');
                        }
                        Err(e) => {
                            eprintln!("Error reading directory entry for {pathbuf:?}: {e}");
                            return None;
                        }
                    }
                }
                Some(listing)
            }
            Err(e) => {
                eprintln!("Error opening dir {pathbuf:?}: {e}");
                None
            }
        }
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> io::Result<()> {
        let len_bytes = bulk_read(&mut stream, 4)?;
        if len_bytes.len() < 4 {
            bulk_write(&mut stream, b"Bad path\n")?;
            return Ok(());
        }

        let len =
            u32::from_be_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

        const MAX_PATH_LEN: usize = 64 * 1024;
        if len == 0 || len > MAX_PATH_LEN {
            bulk_write(&mut stream, b"Bad path\n")?;
            return Ok(());
        }

        let path_bytes = bulk_read(&mut stream, len)?;
        if path_bytes.len() != len {
            bulk_write(&mut stream, b"Bad path\n")?;
            return Ok(());
        }

        let path_string_res = String::from_utf8(path_bytes);
        let pathbuf = match path_string_res {
            Ok(s) => {
                let s = s.trim().to_string();
                match PathBuf::from_str(&s) {
                    Ok(pb) => pb,
                    Err(_) => {
                        bulk_write(&mut stream, b"Bad path\n")?;
                        return Ok(());
                    }
                }
            }
            Err(_) => {
                bulk_write(&mut stream, b"Bad path\n")?;
                return Ok(());
            }
        };

        match self.list_dir(&pathbuf) {
            Some(listing) => bulk_write(&mut stream, listing.as_bytes())?,
            None => bulk_write(&mut stream, b"Bad path\n")?,
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let server = DirectoryServer::new(IP_ADDRESS, PORT);
    server.run()
}
