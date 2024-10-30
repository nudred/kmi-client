use clap::{Arg, ArgGroup, Command};
use colored::Colorize;
use reqwest::blocking::Client;
use std::io::{self, BufRead};

const URL_BASE: &str = "https://kmi.aeza.net";

fn ping_aeza(client: &Client) {
    let url = format!("{}/ping", URL_BASE);
    match client.post(&url).send() {
        Ok(response) => {
            if let Ok(text) = response.text() {
                println!("{}", text.green());
            }
        }
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

fn get_ip(client: &Client) {
    let url = format!("{}/ip", URL_BASE);
    match client.post(&url).send() {
        Ok(response) => {
            if let Ok(text) = response.text() {
                println!("{}", text.green());
            }
        }
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

fn send_data(client: &Client, data: &str) {
    let url = URL_BASE.to_string();
    let params = [("kmi", data)];
    match client.post(&url).form(&params).send() {
        Ok(response) => {
            if let Ok(text) = response.text() {
                println!("{}", text.green());
            }
        }
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

fn main() {
    let client = Client::new();
    let matches = Command::new("kmi")
        .version("0.1.0")
        .about("Yet another PasteBin by Aeza")
        .arg(Arg::new("text").index(1).help("Send text"))
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .action(clap::ArgAction::SetTrue)
                .help("Check your IP"),
        )
        .arg(
            Arg::new("ping")
                .short('p')
                .long("ping")
                .action(clap::ArgAction::SetTrue)
                .help("Ping"),
        )
        .group(ArgGroup::new("req_flags").arg("text").arg("ip").arg("ping"))
        .get_matches();

    if !atty::is(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let data: String = stdin.lock().lines().filter_map(Result::ok).collect();
        send_data(&client, &data);
        return;
    }
    if let Some(text) = matches.get_one::<String>("text") {
        // Получаем значение аргумента "text"
        send_data(&client, text);
        return;
    }
    if matches.get_flag("ip") {
        // Проверка наличия аргумента "ip"
        get_ip(&client);
        return;
    }
    if matches.get_flag("ping") {
        // Проверка наличия аргумента "ping"
        ping_aeza(&client);
        return;
    }
    println!("Mini-client for {}", URL_BASE.yellow())
}
