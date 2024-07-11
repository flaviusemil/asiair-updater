use std::{fs, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use bzip2::Compression;
use bzip2::write::BzEncoder;
use log::{error, info};
use tar::{Builder as TarBuilder, Header};

const PAYLOAD_SCRIPT_NAME: &str = "update_package.sh";

pub fn read_payload_content<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn create_tar_bz2<P: AsRef<Path>>(src_path: P, output_path: &str) -> io::Result<()> {
    if Path::new(output_path).exists() {
        info!("Removing old payload.tar.bz2 at {}", output_path);
        fs::remove_file(output_path).expect("Could not remove file!");
    }

    let tar_bz2_file = File::create(output_path)?;
    let encoder = BzEncoder::new(tar_bz2_file, Compression::best());
    let mut tar = TarBuilder::new(encoder);

    let mut file = File::open(src_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut header = Header::new_gnu();
    header.set_size(buffer.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    tar.append_data(&mut header, PAYLOAD_SCRIPT_NAME, buffer.as_slice())?;
    tar.into_inner()?.finish()?;

    Ok(())
}

pub fn delete_tar_bz2<P: AsRef<Path>>(path: P) {
    match fs::remove_file(path) {
        Ok(()) => info!("Payload cleaned successfully!"),
        Err(e) => error!("There was an error cleaning the payload! {}", e)
    }
}
