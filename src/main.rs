extern crate mio;

use std::net::SocketAddr;
use mio::tcp::*;



struct WebSocketServer;

impl Handler for WebSocketServer{
  type Timeout = usize;
  type Message = ();
}

fn main() {
    println!("Hello, world!");
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    // 토큰은 소켓의 유일한 식별자이다.
    // EventSet은 이벤트 구독에 대한 intent를 설명한다.
    event_loop.register(&server_socket, Token(0),
                        EventSet::readable(), PollOpt::edge()).unwrap();



    // 핸들러 구조체의 새로운 인스턴스를 만든다.
    let mut handler = WebSocketServer;
    // 이벤트 루프에 변할 수 있는 참조를 넘긴다.
    event_loop.run(&mut handler).unwrap();
}
