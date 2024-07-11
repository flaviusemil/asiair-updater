use futures_util::{pin_mut, StreamExt};
use log::info;
use mdns::{Record, RecordKind};
use std::net::IpAddr;
use std::time::Duration;
use crate::errors::AppError;

const ASIAIR_SERVICE_NAME: &str = "_device-info._tcp.local";
const MDNS_QUERY_INTERVAL: Duration = Duration::from_secs(1);

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        _ => None,
    }
}

pub async fn scan_for_asi_ip() -> Option<IpAddr> {
    let stream = mdns::discover::all(ASIAIR_SERVICE_NAME, MDNS_QUERY_INTERVAL)
        .map_err(AppError::ErrorStartingMdnsDiscovery)
        .ok()?
        .listen();

    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        if let Some(addr) = response.records().filter_map(to_ip_addr).next() {
            info!("Found asiair.local device at [{}]", addr);
            return Some(addr);
        }
    }

    None
}
