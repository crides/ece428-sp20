use clap::{App, Arg};
use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};

fn main() -> io::Result<()> {
    let app = App::new("logger")
        .arg(Arg::with_name("name").required(true))
        .arg(Arg::with_name("addr").required(true))
        .arg(Arg::with_name("port").required(true));
    let matches = app.get_matches();
    let name = matches.value_of("name").unwrap();
    let addr = matches.value_of("addr").unwrap();
    let port = matches.value_of("port").unwrap();
    let stdin = io::stdin();
    let mut buf = String::new();
    let server_addr = format!("{}:{}", addr, port)
        .parse::<SocketAddr>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    dbg!(&server_addr);

    while let Ok(size) = stdin.read_line(&mut buf) {
        if size == 0 {
            break;
        }

        let mut splits = buf.splitn(2, " ");
        let (time, msg) = (splits.next().unwrap(), splits.next().unwrap());
        let msg = msg.trim_end();

        let mut stream = TcpStream::connect(server_addr)?;
        write!(stream, "{}|{}|{}", time, name, msg)?;
        buf.clear();
    }
    Ok(())
}
