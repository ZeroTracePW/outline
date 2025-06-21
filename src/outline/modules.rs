use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::outline::connector::{connect_ws, ws_send_message};
use crate::outline::interface::{clear, input, input_with_prompt, logo, write, write_ln};

pub async fn connect_server() -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    clear();
    logo();

    write_ln("1", "Enter Manual Details");
    write_ln("2", "Enter URL (e.g. wss://example.com:8080/)");
    write(">", "");
    let input_type = input();

    let (scheme, host, path) = if input_type == "1" {
        let scheme = input_with_prompt("Scheme (wss/ws): ");
        let ip_domain = input_with_prompt("Enter Domain/IP (e.g. 127.0.0.1/example.com): ");
        let port = input_with_prompt("Enter Port: (e.g 8080):");
        let path = input_with_prompt("Enter Path: (e.g /):");
        let host = format!("{}:{}", ip_domain, port);
        (scheme, host, path)
    } else {
        let url = input_with_prompt("Enter URL: ");
        parse_url(&url).unwrap_or_else(|err| {
            write_ln("-", format!("Error: {}", err).as_str());
            std::process::exit(1);
        })
    };

    connect_ws(scheme.as_str(), host.as_str(), path.as_str())
        .await
        .unwrap_or_else(|err| {
            write("-", format!("Error: {}", err).as_str());
            let _ = input();
            std::process::exit(0);
        })
}

fn parse_url(url: &str) -> Result<(String, String, String), String> {
    let url_parts: Vec<&str> = url.split("://").collect();
    if url_parts.len() != 2 {
        return Err("Invalid URL format".to_string());
    }
    let (scheme, rest) = (url_parts[0].to_string(), url_parts[1]);

    let rest_parts: Vec<&str> = rest.splitn(2, '/').collect();
    if rest_parts.len() != 2 {
        return Err("Invalid URL format".to_string());
    }
    let (host, path) = (rest_parts[0], format!("/{}", rest_parts[1]));

    let host_parts: Vec<&str> = host.split(':').collect();
    if host_parts.len() != 2 {
        return Err("Invalid URL format".to_string());
    }
    let (ip_domain, port) = (host_parts[0], host_parts[1]);
    let host = format!("{}:{}", ip_domain, port);

    Ok((scheme, host, path))
}

pub async fn connect_handshake() {
    let mut ws = connect_server().await;

    clear();
    write_ln("?", "type 'q' to quit");
    write_ln("?", "type 'c' to clear screen");
    loop {
        let message = input_with_prompt("Enter Message: ");

        match message.as_str() {
            "q" => break,
            "c" => {
                clear();
                continue;
            },
            _ => {
                write_ln("+", format!("Message Sent: {}", message).as_str());
                ws_send_message(&mut ws, message.as_str()).await;
                println!();
            },
        }
    }
}