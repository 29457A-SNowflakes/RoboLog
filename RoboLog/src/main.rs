use std::*;

use clap::{Command, Arg};
use tokio;


async fn ListDevices(arg: String) {
    println!("Listing {} devices", arg);
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
                        .value_parser(["all", "bluetooth", "direct", "controller"])
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
