use std::net::IpAddr;

use clap::{arg, Parser};
use log::{error, info};
use serde_json::json;

use errors::AppError;
use mdns_discovery::scan_for_asi_ip;
use payload::{create_tar_bz2, read_payload_content};
use utils::calculate_md5;

use crate::network::{try_socket_connect, write_data_to_server};
use crate::utils::recv_all;

mod errors;
mod logger;
mod mdns_discovery;
mod network;
mod payload;
mod utils;

const PAYLOAD_FILE_PATH: &str = "scripts/payload.sh";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: Option<IpAddr>,
}

#[async_std::main]
async fn main() -> Result<(), AppError> {
    logger::init_logger();
    let args = Args::parse();

    let mut ip = args.ip;

    if let Some(ip) = ip {
        info!("Using provided IP address: {}", ip);
    } else {
        info!("No IP address provided, scanning the network for ASI Air...");
        ip = scan_for_asi_ip().await;
    }

    let ip = match ip {
        Some(ip) => ip,
        None => return Err(AppError::ErrorFindingMdnsIP),
    };

    let payload_path = "scripts/payload.tar.bz2";
    match create_tar_bz2(PAYLOAD_FILE_PATH, payload_path) {
        Ok(()) => info!("Created tar.bz2 archive successfully!"),
        Err(e) => error!("Failed to create tar.bz2 archive: {}", e)
    };

    info!("Loading payload in memory...");
    let payload_content = read_payload_content(payload_path).ok().ok_or(AppError::ErrorReadingPayloadFile)?;
    let payload_size = payload_content.len();
    let payload_hash = calculate_md5(&payload_content);

    let json_payload = json!({
        "id": 1,
        "method": "begin_recv",
        "params": [{
            "file_len": payload_size,
            "file_name": "air",
            "run_update": true,
            "md5": payload_hash
        }]
    });

    payload::delete_tar_bz2(payload_path);

    let payload = format!("{}\r\n", serde_json::to_string(&json_payload).unwrap());

    let ota_sock = try_socket_connect(ip, network::OTA_PORTS).unwrap();
    let command_sock = try_socket_connect(ip, &[network::COMMAND_PORT]).unwrap();

    info!("Got: {}", recv_all(command_sock.try_clone().unwrap()).unwrap().trim());
    info!("Sending RPC: {}", payload.trim());

    if let Err(err) = write_data_to_server(command_sock.try_clone().unwrap(), payload.as_bytes()) {
        error!("There was an error: {}", err);
    } else {
        info!("Payload content sent successfully.");
    }

    match recv_all(command_sock) {
        Ok(resp) => info!("Got back: {}", resp.trim()),
        Err(e) => error!("Error receiving response: {}", e)
    }

    if let Err(err) = write_data_to_server(ota_sock, payload.as_bytes()) {
        error!("There was an error while sending the payload: {}", err);
    } else {
        info!("Payload was sent successfully!")
    }
    Ok(())
}
