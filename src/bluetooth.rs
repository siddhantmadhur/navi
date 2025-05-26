use std::sync::Arc;

use futures::lock::Mutex;

pub struct BluetoothDevice {
    address: String,
    name: Option<String>,
}

pub fn list_devices() -> Arc<Mutex<Vec<BluetoothDevice>>> {
    let devices = Arc::new(Mutex::new(vec![]));
    
    return devices;
}
