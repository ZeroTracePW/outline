pub mod connector {
    use futures::{SinkExt, StreamExt};
    use std::error::Error;
    use tokio::net::TcpStream;
    use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
    use tungstenite::http::status;
    use tungstenite::{Message, Utf8Bytes};
    use url::Url;
    use crate::outline::interface::write_ln;

    pub async fn connect_ws(scheme: &str, host: &str, path: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
        let url = format!("{}://{}{}", scheme, host, path);
        let url = Url::parse(&url)?;

        let (socket, response) = match connect_async(format!("{}", url)).await {
            Ok(result) => result,
            Err(e) => {
                write_ln("-", e.to_string().as_str());
                return Err(Box::new(e));
            },
        };

        if response.status() != status::StatusCode::SWITCHING_PROTOCOLS {
            Err("Invalid response status")?;
        }

        let addr = match socket.get_ref() {
            MaybeTlsStream::Plain(stream) => stream.peer_addr().ok(),
            MaybeTlsStream::NativeTls(stream) => stream.get_ref().get_ref().get_ref().peer_addr().ok(),
            _ => None,
        };

        let addr = addr.ok_or("Failed to get peer address")?;

        write_ln("IP",  addr.ip().to_string().as_str());
        write_ln("Port",  addr.port().to_string().as_str());
        write_ln("+", format!("Connected to {}", url).as_str());

        Ok(socket)
    }

    pub async fn ws_send_message(socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, message: &str) {
        let (mut write, mut read) = socket.split();

        let message = message.to_string();
        let message = Message::Text(Utf8Bytes::from(message));

        if let Err(error) = write.send(message.clone()).await {
            write_ln("-", error.to_string().as_str());
            return;
        }

        match read.next().await {
            Some(Ok(msg)) => write_ln("+", format!("Received: {}", msg).as_str()),
            Some(Err(error)) => {
                write_ln("-", "Failed to read message");
                write_ln("-", error.to_string().as_str());
            },
            None => write_ln("-", "No message received"),
        }
    }

    #[warn(dead_code)]
    pub async fn get_website_body(url: &str) -> Result<String, Box<dyn Error>> {
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;

        Ok(body)
    }
}