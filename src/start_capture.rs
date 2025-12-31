use anyhow::*;
use etherparse::PacketHeaders;
use log::warn;
use std::{error::Error, sync::{mpsc::{self, Receiver, Sender}, OnceLock}, thread};

use crate::packets::opcodes::Pkt;

pub trait PacketCapture: std::fmt::Debug + Send + Sync + 'static {
    fn start(
        &self,
        port: u16,
        region_file_path: String,
    ) -> Result<Receiver<(Pkt, Vec<u8>)>>;
}

#[derive(Debug)]
struct NoopPacketCapture;

impl PacketCapture for NoopPacketCapture {
    fn start(
        &self,
        _port: u16,
        _region_file_path: String,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<(Pkt, Vec<u8>)>> {
        let (_tx, rx) = std::sync::mpsc::channel();
        Ok(rx)
    }
}

static PACKET_CAPTURE_IMPL: OnceLock<Box<dyn PacketCapture>> = OnceLock::new();

pub fn set_packet_capture_impl<C: PacketCapture>(capture: C) {
    PACKET_CAPTURE_IMPL
        .set(Box::new(capture))
        .expect("PacketCapture implementation already set");
}

pub fn start_capture(port: u16, region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>> {
    let capture = PACKET_CAPTURE_IMPL.get_or_init(|| {
        warn!(
            "PacketCapture implementation not registered; Register one via set_packet_capture_impl(...) before calling start_capture(). defaulting to no-op capture."
        );
        Box::new(NoopPacketCapture)
    });

    capture.start(port, region_file_path)
}