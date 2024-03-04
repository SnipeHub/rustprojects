/* Our dependencies go here */ 
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;
use tokio::time;
use std::thread::Builder;

/* Scan a single port asynchronously */
async fn scan_port(target: IpAddr, port: u16, timeout: u64)
{
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match time::timeout(timeout, TcpStream::connect(&socket_address)).await
    {
        Ok(Ok(_)) => println!("Port {port} is open"),
        _ => {}
    }
}

/* Scan a range of ports concurrently */
async fn scan_ports(target: IpAddr, start_port: u16, end_port: u16, timeout: u64)
{
    for port in start_port..=end_port
    {
        scan_port(target, port, timeout).await;
    }
}

/* Handle incoming client connections, need to build out */
fn handle_client(stream: TcpStream) {}

fn main() 
{
    let target = "127.0.0.1".parse().expect("Invalid IP address");
    let start_port = 80;
    let end_port = 1024;
    let timeout = 3;

    // Start listening for incoming connections
    let listener = TcpListener::bind(format!("{}:8080", target))
        .expect("Error binding to address");

    println!("Listening on {}:8080...", target);

    /* Accept incoming connections and handle them */
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                // spawn a new thread or use an async runtime to handle each client
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {e}");
            }
        }
    }

    /* Start scanning ports */
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(scan_ports(target, start_port, end_port, timeout));
}