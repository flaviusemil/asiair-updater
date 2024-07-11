use mdns::Error as MdnsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to start mDNS discovery: {0}")]
    ErrorStartingMdnsDiscovery(#[from] MdnsError),

    #[error("Could not connect to any of the specified ports: {0:?}")]
    ErrorConnectingToSocketPorts(Vec<u16>),

    #[error("No IP address found for asiair.local.")]
    ErrorFindingMdnsIP,

    #[error("There was an error trying to read the content of the payload file.")]
    ErrorReadingPayloadFile,
}
