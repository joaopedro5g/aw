mod connection;
use connection::{handle_client, send_route};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let listener = TcpListener::bind("0.0.0.0:8080").await?;
  loop {
    match listener.accept().await {
      Ok((mut socket,_)) => {
        tokio::spawn(async move {
          let (read, write) = socket.split();
          match handle_client(read).await {
            Ok(test) => {
              println!("Resultado foi {test} do cliente");
            },
            Err(e) => {
              panic!("Pani no sistema, alguém me desconfigurou!");
            }
          }
          match send_route(write, "/api").await {
            Ok(_) => {},
            Err(_) => {}
          }
        });
      },
      Err(_) => {
        eprintln!("Erro ao iniciar o servidor tcp");
      }
    };
  }
}