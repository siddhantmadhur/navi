use std::{
    fs,
    io::{self, BufRead},
    os::unix::net::{UnixListener, UnixStream},
    sync::Arc, thread::sleep,
};

use futures::lock::Mutex;

pub struct BluetoothDevice {
    address: String,
    name: Option<String>,
}

pub fn list_devices() -> Vec<BluetoothDevice> {
    let message = "hello world\n";
    let interval = std::time::Duration::from_secs(1);

    let mut stream = UnixStream::connect("/tmp/navi/bluetooth.sock").unwrap();

    return vec![];
}

pub async fn create_background_service() {
    let path = "/tmp/navi/bluetooth.sock";
    let _ = fs::remove_dir_all("/tmp/navi");
    let _ = fs::create_dir("/tmp/navi");
    let listener = UnixListener::bind(path).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = stream.try_clone().unwrap();
        let reader = io::BufReader::new(reader);

        eprintln!("pong: stream");
        for response in reader.lines() {
            let response = response.unwrap();
            println!("Response: {}", response);
        }
    }
}
