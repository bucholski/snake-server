pub mod move_management;
use move_management::*;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
type DirQueue = Arc<Mutex<Vec<Direction>>>;

#[tokio::main]
async fn main() {
    let move_queue: DirQueue = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let move_queue = move_queue.clone();
        println!("Accepted");
        tokio::spawn(async move {
            handle_connection(stream, move_queue).await;
        });
    }

    async fn handle_connection(mut stream: TcpStream, queue: DirQueue) {
        let mut buf = [0; 1024];
        let http_request = match stream.read(&mut buf).await {
            // Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        // .lines()
        // .map(|result| result.unwrap())
        // .take_while(|line| !line.is_empty())
        // .collect();

        // let get_direction = match http_request.get(0) {
        // Some(a) => a,
        // None => "Nothing",
        // };

        // match get_direction {
        //     "POST /snake/right HTTP/1.1" => queue.push(Direction::Right),
        //     "POST /snake/left HTTP/1.1" => queue.push(Direction::Left),
        //     "POST /snake/up HTTP/1.1" => queue.push(Direction::Up),
        //     "POST /snake/down HTTP/1.1" => queue.push(Direction::Down),
        //     _ => eprintln!("Error: Bad Request"),
        // }

        // println!("{:#?}", http_request);

        let status_line = "HTTP/1.1 200 OK";
        let contents = "debug time!";
        let length = contents.len();
        println!();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // stream.write_all(response.as_bytes()).unwrap();
    }
}
