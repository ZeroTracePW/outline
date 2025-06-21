use crate::outline::interface::{input, logo, set_title, write, write_ln};
use crate::outline::modules::modules::connect_handshake;

mod outline {
    pub mod interface;
    pub mod connector;
    pub mod modules;
}

#[tokio::main]
async fn main() {
    set_title("Outline |  Powered by ZeroTrace (zerotrace.pw)");
    logo();
    
    write_ln("1", "Connect and Handshake");
    write_ln("2", "Basic Fuzzing");
    write_ln("3", "Message-based Vulnerability Detection");
    write_ln("0", "Exit");
    write(">", "");

    let input = input();

    match input.as_str() {
        "1" => {
            connect_handshake().await;
        }
        _ => {
            write_ln("-", "Invalid Input");
        }
    }
}