use bluer::{gatt::remote::Characteristic, AdapterEvent, Device, Result};
use futures::{pin_mut, StreamExt};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};

mod bluetooth;

#[cfg(not(target_os = "linux"))]
compile_error!("This program is only designed for Linux.");


#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {

    let session = bluer::Session::new().await.unwrap();
    let adapter = session.default_adapter().await.unwrap();
    adapter.set_powered(true).await.unwrap();

    {
        println!("Discovering devices on {} with address {}\n", adapter.name(), adapter.address().await.unwrap());
        let discover = adapter.discover_devices().await?;
        pin_mut!(discover);
        let mut done = false;
        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr).unwrap();
                    let device_name = device.name().await.unwrap();
                    match device_name {
                        Some(real_name) => {
                            println!("[{}] {}", addr, real_name);
                        },
                        _=>()
                    }
                },
                _=>()
            } 
        }
    }


    let devices = bluetooth::list_devices();

    println!("Devices: {}", devices.len());

    return Ok(());
}
