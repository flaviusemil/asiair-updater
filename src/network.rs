use std::io;
use std::io::{Write};
use std::net::{IpAddr, TcpStream};

use log::{error, info};
use crate::errors::AppError;

pub(crate) const OTA_PORTS: &[u16] = &[4360, 4361];
pub(crate) const COMMAND_PORT: u16 = 4350;

pub fn try_socket_connect(ip: IpAddr, ports: &[u16]) -> Result<TcpStream, AppError> {
    for &port in ports {
        let address = format!("{}:{}", ip, port);
        match TcpStream::connect(&address) {
            Ok(stream) => {
                info!("Successfully connected to server at {}", address);
                return Ok(stream);
            }
            Err(e) => {
                error!("Failed to connect to {}: {}", address, e);
            }
        }
    }
    Err(AppError::ErrorConnectingToSocketPorts(ports.to_vec()))
}

pub fn write_data_to_server(mut stream: TcpStream, data: &[u8]) -> io::Result<()> {
    stream.write(data)?;
    stream.flush()?;
    Ok(())
}
