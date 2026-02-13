
// tcp client
use std::net::TcpStream;
let stream = TcpStream::connect("127.0.0.1:8000").unwrap();

// tcp server
use std::net::TcpListener;
let stream = TcpListener::bind("127.0.0.1:8000").unwrap();

// udp
use std::net::UdpSocket;
let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();
// send_to() - send msg to specific address
// recv_from() - receive msg and returns the source address

