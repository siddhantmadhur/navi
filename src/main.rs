#[cfg(not(target_os = "linux"))]
compile_error!("This program is only designed for Linux.");

use bluer::{gatt::remote::Characteristic, AdapterEvent, Device, Result};
use clap::{Args, Parser, Subcommand};
use core::time;
use futures::{pin_mut, StreamExt};
use std::{
    error::Error,
    fs::{self, File},
    io::{stdin, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{stdout, AsyncReadExt, AsyncWriteExt},
    net::UnixSocket,
    time::sleep,
};
mod bluetooth;

#[derive(Parser)]
#[clap(name = "navi")]
struct App {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum BluetoothCommands {
    /// search for new devices
    Scan {
        /// "on"/"off"
        enabled: String,
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
    Network { command: String },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let app = App::parse();

    match app.command {
        Some(command) => {
            match command {
                Command::Bluetooth { commands } => match commands {
                    BluetoothCommands::Scan { enabled } => match enabled.as_str() {
                        "on" => {
                            print!("bluetooth enabled: {} \n", enabled);

                            let f = File::open("/tmp/navi/devices");
                            match f {
                                Ok(mut f) => {
                                    let mut str = String::new();
                                    let _ = f.read_to_string(&mut str);
                                    print!("Output: {}\n", str);
                                }
                                Err(_) => {
                                    print!("Daemon is not active\n");
                                }
                            }
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
            }
        }
        None => {
            print!("Background process...\n");
            //fs::remove_dir_all("/tmp/navi").unwrap();
            //fs::create_dir_all("/tmp/navi/").unwrap();

            let _ = fs::create_dir_all("/tmp/navi");
            let mut f = File::create("/tmp/navi/devices").unwrap();
            loop {
                let _ = f.write(b"Device 1\n");
                let _ = sleep(time::Duration::from_secs(2)).await;
            }
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
