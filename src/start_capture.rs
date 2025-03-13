use std::{error::Error, sync::{mpsc::{self, Receiver, Sender}, Mutex}};

use crate::packets::opcodes::Pkt;

pub static SENDER: Mutex<Option<Sender<(Pkt, Vec<u8>)>>> = Mutex::new(None);

pub fn start_capture(_port: u16, _region_file_path: String) -> Result<Receiver<(Pkt, Vec<u8>)>, Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<(Pkt, Vec<u8>)>();

    let _ = SENDER.lock().unwrap().insert(tx);

    Ok(rx)
}