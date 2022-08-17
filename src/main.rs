extern crate serde_derive;

mod decoder;
mod structs;

// modules
use decoder::Decoder;
use structs::Args;

// extern creates
use clap::Parser;

use dump_dvb::receivers::RadioReceiver;

// standard lib
use std::fs::read_to_string;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};

#[tokio::main]
async fn main() {
    const BUFFER_SIZE: usize = 2048;
    let args = Args::parse();

    let stop_mapping = String::from(&args.config);
    let contents = read_to_string(stop_mapping).expect("Something went wrong reading the file");

    let station_config: RadioReceiver =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let decoder = Decoder::new(&station_config, &args.server).await;

    println!("Starting DVB Dump Telegram Decoder ... ");
    let addr = format!("{}:{}", &args.host, &args.port);
    let socket = UdpSocket::bind(addr).unwrap();
    let (tx, rx): (SyncSender<[u8; BUFFER_SIZE]>, Receiver<[u8; BUFFER_SIZE]>) =
        mpsc::sync_channel(400);

    let _thread = tokio::spawn(async move {
        loop {
            let data = rx.recv().unwrap();
            decoder.process(&data).await;
        }
    });
    loop {
        let mut buffer = [0; BUFFER_SIZE];
        let (_amt, _src) = socket.recv_from(&mut buffer).unwrap();
        tx.send(buffer.clone()).unwrap();
    }
}
