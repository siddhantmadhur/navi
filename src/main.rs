#[cfg(not(target_os = "linux"))]
compile_error!("This program is only designed for Linux.");

use bluer::{gatt::remote::Characteristic, AdapterEvent, Device, Result};
use clap::{Args, Parser, Subcommand};
use futures::{pin_mut, StreamExt};
use std::{error::Error, io::stdin, process::exit, sync::Arc, time::Duration};
use tokio::{
    io::{stdout, AsyncReadExt, AsyncWriteExt},
    time::sleep,
};
mod bluetooth;


#[derive(Parser)]
#[clap(name = "navi")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum BluetoothCommands {
    /// search for new devices
    Scan { 
        /// "on"/"off" 
        enabled: String 
    },
}

#[derive(Subcommand)]
enum Command {
    /// manage bluetooth devices
    Bluetooth {
        #[clap(subcommand)]
        commands: BluetoothCommands,
    },
    /// manage network devices 
    Network {
        command: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let app = App::parse();

    match app.command {
        Command::Bluetooth { commands } => match commands {
            BluetoothCommands::Scan { enabled } => match enabled.as_str() {
                "on" => {
                    print!("bluetooth enabled: {} \n", enabled);
                }
                "off" => {
                    print!("bluetooth disabled: {} \n", enabled);
                }
                _ => {
                    print!("invalid bluetooth command\n");
                    exit(1);
                }
            },
        },
        Command::Network { command } => {
            print!("Network...{} \n", command);
        }
        _ => {
            print!("Something else\n");
        }
    }
    /**

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


    let devices_arc = bluetooth::list_devices();

    {
    let devices_arc = Arc::clone(&devices_arc);
    let devices = devices_arc.lock().await;
    println!("Devices: {}", devices.len());
    }
    */
    return Ok(());
}
