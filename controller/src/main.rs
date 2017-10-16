use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;

const RESPONSE: &'static [u8] = b"HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r\n\r
<!DOCTYPE html>
<html>
<head>
<title>controller</title>
</head>
<body>done</body>
</html>";

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

  for stream in listener.incoming() {
    thread::spawn(move || {
      let mut stream = stream.unwrap();
      match stream.write(RESPONSE) {
        Ok(_) => println!("Response sent!"),
        Err(e) => eprintln!("Failed sending response: {}!", e),
      }
      stream.shutdown(Shutdown::Write).unwrap();
    });
  }
}
