use anyhow::*;
use etherparse::PacketHeaders;
use std::{error::Error, sync::mpsc::{self, Receiver, Sender}, thread};

use crate::packets::opcodes::Pkt;

pub fn start_capture(_port: u16, _region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>> {
    let (tx, rx) = mpsc::channel::<(Pkt, Vec<u8>)>();
    Ok(rx)
}