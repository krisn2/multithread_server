use std::{
 fs,
  io::{prelude::*, BufReader},
   net::{TcpListener, TcpStream} ,
   thread,
   time::Duration,
};
use multithread_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(2);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{:#?}",request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}
    // let http_req:Vec<_> = buf_reader
    //     .lines()
    //     .map(|result|result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // let request_line = http_req[0].to_string();

    // if request_line == "GET / HTTP/1.1" {
    // let status = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("src/hello.html").unwrap();
    // let length = contents.len();

    // let response = 
    // format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();
    // }
    // else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("src/404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // }

//     let res = "HTTP/1.1 200 OK\r\n\r\n";
//    println!("{:#?}", http_req);
//    stream.write_all(res.as_bytes()).unwrap();

// let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//     ("HTTP/1.1 200 OK", "hello.html")
// } else {
//     ("HTTP/1.1 404 NOT FOUND", "404.html")
// };

// let contents = fs::read_to_string(filename).unwrap();
// let length = contents.len();

// let response =
//     format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

// stream.write_all(response.as_bytes()).unwrap();
