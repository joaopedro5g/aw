use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{ TcpListener, TcpStream }};

#[warn(unreachable_patterns)]
async fn handle_client(mut socket: TcpStream) {
  let mut buf = [0;1024];
  loop {
    match socket.read(&mut buf).await {
      Ok(O) => {
        println!("Client Desconectado");
        break;
      },
      Ok(n) => {
        println!("Mensagem recebida {}", String::from_utf8_lossy(&buf[..n]));
        if let Err(e) = socket.write_all(&buf[..n]).await {
          eprintln!("Erro ao escrever no socket {}", e);
          break;
        }
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
    let (socket, _) = listener.accept().await?;
    tokio::spawn(async move {
      handle_client(socket)
    });
  }
}