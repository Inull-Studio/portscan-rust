use clap::Parser;
use rayon::ThreadPoolBuilder;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(version="1.0", about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg()]
    host: String,
    /// ports
    #[arg(short, long, default_value_t = String::from("21,22,23,25,80"))]
    ports: String,
    #[arg(short, long, default_value_t = 500)]
    threads: i32,
}
fn main() {
    let args = Args::parse();
    let mut ports: Vec<i32> = Vec::new();
    println!("host: {}", args.host);
    println!("ports: {}", args.ports);
    let port_s = args.ports.split(",");
    for port in port_s {
        if port.contains("-") {
            let mut t_ports: Vec<i32> = Vec::new();
            let mut ps = port.split("-");
            let sp = ps.next().unwrap().parse().unwrap();
            let ep = ps.last().unwrap().parse().unwrap();
            for i in sp..=ep {
                t_ports.push(i);
            }
            ports.append(&mut t_ports);
        } else {
            ports.push(port.parse().unwrap());
        }
    }
    let pool = ThreadPoolBuilder::new()
        .num_threads(args.threads.try_into().unwrap())
        .build()
        .unwrap();
    for port in ports {
        let socket = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::from_str(&args.host).unwrap()),
            port.try_into().unwrap(),
        );
        pool.spawn(move || {
            let stream = TcpStream::connect_timeout(&socket, Duration::from_millis(1000));
            if stream.is_ok() {
                println!("Open {}", port);
            } else {
                // println!("No {}", port);
            }
            drop(stream);
        });
    }
    pool.join(|| {}, || {});
}
