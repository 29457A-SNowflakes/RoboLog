use std::*;
use connection::serial::{find_devices, SerialDevice};
use vex_v5_serial::*;
use clap::{Command, Arg};
use tokio;


async fn ListDevices(arg: String) {
    if arg == "direct" || arg == "all" {    
        let results = find_devices();
        match results {
            Err(e) => {
                println!("Issue finding devices: {}", e.to_string());
                return;
            },
            Ok(devices) => {
                println!("{} device(s) found", devices.len());
                if devices.is_empty() {return;}
                else {print!(":")}
                for device in devices.iter() {
                    match device {
                        SerialDevice::Brain { user_port: _, system_port } => {
                            println!("Brain on {}", system_port);
                        },
                        SerialDevice::Controller { system_port } => {
                            println!("Controller on {}", system_port);
                        },
                        SerialDevice::Unknown { system_port } => {
                            println!("Unknown device on {}", system_port);
                            return;
                        }
                    }
                }
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let cli = Command::new("robolog")
        .author("Daniel Dew, V5RC 29457A")
        .about("Recieves packets from V5 brain and logs to mqtt broker!")
        .subcommand(
            Command::new("list-devices")
                .about("Lists all connected V5 devices")
                .arg(
                    Arg::new("type")
                        .long("type")
                        .short('t')
                        .help("The type of connection")
                        .value_name("TYPE")
                        .required(false)
                        .default_value("all")
                        .value_parser(["all", "bluetooth", "direct"])
                        .ignore_case(true)
                )
        );
    let args = cli.clone().get_matches();
    match args.subcommand_name() {
        None => {
            println!("!");
        }
        Some(name) => {
            if let name = "list devices" {
                let arg = args.subcommand().unwrap().1.get_one::<String>("type").unwrap();
                ListDevices(arg.to_string()).await;
            }
        }
    }
}
