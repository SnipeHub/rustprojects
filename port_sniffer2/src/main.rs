// Using Rust 1.78
use std::io::{self, Error}; // Import io module and Error enum
use std::net::IpAddr;
use std::time::Duration;
use std::os::unix::net::SocketAddr;

#[macro_use]
use tokio; // Import the entire tokio crate
use tokio::{
    net::{TcpStream},
    task,
};

// Define an asynchronous function to connect and send data to a port
async fn connect_to_port(target: IpAddr, port: u16) -> Result<(), io::Error> {
    let target = SocketAddr::new(target, port);
    let mut stream = match TcpStream::connect_async(&target).await {
        Ok(stream) => stream,
        Err(e) => return Err(e),
    };

    // Send data to the port
    let out_con = b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    match stream.write_all(&out_con).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // Read the response data from the port
    let mut in_con = Vec::new();
    let bytes_read = match stream.read(&mut in_con).await {
        Ok(bytes_read) => bytes_read,
        Err(e) => return Err(e),
    };

    println!("Bytes read: {}", bytes_read);

    // Ensure the connection is closed after use
    match stream.shutdown(Duration::from_secs(5)).await {
        Ok(_) => (),
        Err(e) => eprintln!("Error shutting down stream: {:?}", e),
    }

    Ok(())
}

// Main function to run the asynchronous connect_to_port function
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let ip = IpAddr::V4(127_0_0_1); // localhost IPv4 address
    match connect_to_port(ip, 80).await {
        Ok(_) => println!("Successfully connected and sent data to port 80"),
        Err(e) => eprintln!("Error connecting to port 80: {:?}", e),
    }

    Ok(())
}