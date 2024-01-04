use std::net::{SocketAddrV4, Ipv4Addr};
use socket2::{Socket, Domain, Type};

pub fn block_port(port: u16) -> Result<(), std::io::Error> {
    let socket = Socket::new(
        Domain::IPV4, 
        Type::STREAM, 
        None
    )?;
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(loopback, port);

    socket.bind(&addr.into())?;

    println!("Socket is bound to loopback address.");
    Ok(())
}

pub fn block_ip(ip: Ipv4Addr) -> Result<(), std::io::Error> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let addr = SocketAddrV4::new(ip, 0);

    socket.bind(&addr.into())?;

    println!("Socket is bound to loopback address.");
    Ok(())
}