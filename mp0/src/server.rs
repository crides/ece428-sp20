use chrono::Local;
use clap::{App, Arg};
use std::io::{self, Read};
use std::net::TcpListener;

fn now() -> f64 {
    Local::now().timestamp_nanos() as f64 / 1e9f64
}

fn main() -> io::Result<()> {
    let app = App::new("logger").arg(Arg::with_name("port").required(true));
    let matches = app.get_matches();
    let port = matches.value_of("port").unwrap();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    let mut resp = String::new();
    for request in listener.incoming() {
        let mut stream = request?;
        let read_size = stream.read_to_string(&mut resp)?;
        let mut splits = resp.split("|");
        let (send_time, name, msg) = (
            splits.next().unwrap(),
            splits.next().unwrap(),
            splits.next().unwrap(),
        );
        let recv_time = now();
        let time_diff = recv_time - send_time.parse::<f64>().unwrap();
        println!("{} - {} connected", recv_time, name);
        eprintln!("[{}] delay: {}, size: {}", recv_time, time_diff, read_size);
        println!("{} msg: {}", now(), msg);
        std::mem::drop(stream);
        println!("{} - {} disconnected", now(), name);
        resp.clear();
    }
    Ok(())
}
