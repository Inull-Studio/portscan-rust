mod threadpool;

use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(version="1.0", about, long_about = None)]
struct Args {
    /// IP地址
    #[arg()]
    host: String,
    /// 端口列表
    #[arg(short, long, default_value_t = String::from("21,22,23,25,80,135,445,3389,7890,8080"))]
    ports: String,
    /// 线程数
    #[arg(short, long, default_value_t = 100)]
    threads: i32,
}
fn main() {
    let args = Args::parse();
    let mut ports: Vec<i32> = Vec::new();
    let host = args.host;
    println!("host: {}", host);
    println!("ports: {}", args.ports);
    println!("threads: {}", args.threads);
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
    let start = Instant::now();
    let opennum = Arc::new(Mutex::new(0));
    {
        let pool = threadpool::ThreadPool::new(args.threads.try_into().unwrap());
        for port in ports {
            let socket =
                SocketAddr::new(IpAddr::from_str(&host).unwrap(), port.try_into().unwrap());
            let opennum = Arc::clone(&opennum);
            pool.exec(Box::new(move || {
                let stream = TcpStream::connect_timeout(&socket, Duration::from_millis(2000));
                if stream.is_ok() {
                    println!("Open {}", port);
                    let mut num = opennum.lock().unwrap();
                    *num += 1;
                }
            }));
        }
    }
    println!("Open port numbers: {}", *opennum.lock().unwrap());
    let end = Instant::now();
    println!("spend: {:?}", end - start);
}
