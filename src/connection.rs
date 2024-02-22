use std::io::Error;

use tokio::{ io::{AsyncReadExt, AsyncWriteExt}, net::tcp::{ ReadHalf, WriteHalf} };

pub async fn send_route(mut stream: WriteHalf<'_>, path: &str) -> Result<(), Error> {
  if let Err(e) = stream.write_all(path.as_bytes()).await {
    eprintln!("Erro");
    return Err(e);
  }
  Ok(())
}

pub async fn handle_client(mut stream: ReadHalf<'_>) -> Result<String, Error> {
  println!("Nova conexão: {}", stream.peer_addr().unwrap());
  let mut buf = [0;1024];
  loop {
    match stream.read(&mut buf).await {
      Ok(n) if n > 0 => {
        let data = String::from_utf8_lossy(&buf[..n]).to_string();
        return Ok(data);
      },
      Ok(_) => {},
      Err(e) => {
        eprintln!("Erro ao ler o socket: {}", e);
        return Err(e);
      }
    }
  }
}