use std::{io, net::TcpStream};
use std::io::Read;

pub fn calculate_md5(bytes: &[u8]) -> String {
    let hash = md5::compute(bytes);
    format!("{:x}", hash)
}

pub(crate) fn recv_all(mut stream: TcpStream) -> io::Result<String> {
    let mut text = String::new();

    loop {
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        text.push_str(&String::from_utf8_lossy(&buffer[..n]));

        if text.ends_with('\n') {
            break;
        }
    }

    Ok(text)
}
