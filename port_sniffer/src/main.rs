/* Our dependencies go here */ 
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;
use tokio::time::timeout;
use std::error::Error;
use tokio::task;

/* Scan a single port asynchronously */
async fn scan_port(target: IpAddr, port: u16, timeout: u64)
{
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match timeout(timeout, socket_address).await
    {
        Ok(Ok(_)) => println!("Port {port} is open"),
    Err(_) => {},
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

/* Handle incoming client connections */
fn handle_client(mut stream: TcpStream) 
{
    let mut in_con = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut in_con).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("Captured data: {:?}", &in_con[..bytes_read]);
    }
}

/* Data sniffing section */
async fn handle_ip_data(listener: TcpListener, target: IpAddr) {
    /* Accept incoming connections and handle them */
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Capturing data from {}", stream.peer_addr().unwrap());

                /* Capture data here... */
                loop {
                    let mut in_con2 = [0 as u8; 1024];
                    let bytes_received = stream.read(&mut in_con2).expect("Failed to read from stream");

                    if bytes_received == 0 {
                        break;
                    }

                    println!("Captured {} bytes of data", bytes_received);
                }
            },
            Err(e) => {
                eprintln!("Error accepting incoming connection: {}", e);
            },
        };
    }
}

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