use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{ TcpListener, TcpStream }};
use serde::{ Serialize ,Deserialize };

#[derive(Serialize, Deserialize)]
struct ClientMessage {
  path: String,
}

async fn handle_client(mut stream: TcpStream) {
  println!("Nova conexão: {}", stream.peer_addr().unwrap());
  let mut buf = [0;1024];
  loop {
    match stream.read(&mut buf).await {
      Ok(n) if n > 0 => {
        println!("Mensagem recebida {}", String::from_utf8_lossy(&buf[..n]));
        if let Err(e) = stream.write_all(&buf[..n]).await {
          eprintln!("Erro ao escrever no socket {}", e);
          break;
        }
      },
      Ok(_) => {
        println!("Client Desconectado");
        break;
      },
      Err(e) => {
        eprintln!("Erro ao ler o socket: {}", e);
        break;
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let listener = TcpListener::bind("0.0.0.0:8080").await?;

  loop {
    match listener.accept().await {
      Ok((mut socket,_)) => {
        tokio::spawn(async move {
          let message = serde_json::to_string(&ClientMessage {
            path: "/user/33123213121".to_string(),
          }).unwrap();
          if let Err(e) = socket.write_all(message.as_bytes()).await {
            eprintln!("Erro ao enviar o JSON para o cliente");
            return;
          }
          handle_client(socket).await;
        });
      },
      Err(_) => {
        eprintln!("Erro ao iniciar o servidor tcp");
      }
    };
  }
}