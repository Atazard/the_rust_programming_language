#[derive(Clone, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => todo!(),
            Message::Move { x, y } => todo!(),
            Message::Write(string) => todo!(),
            Message::ChangeColor(r, g, b) => todo!(),
        }
    }

    fn next(&self) -> Option<Self> {
        // doesn't mean anything, just something to try Option and Some
        match self {
            Message::Quit => Some(Message::Move { x: 0, y: 0 }),
            Message::Move { x, y } => Some(Message::Write(String::from("moved to new position"))),
            Message::Write(string) => Some(Message::ChangeColor(0, 0, 0)),
            Message::ChangeColor(r, g, b) => Some(Message::Quit),
        }
    }
}

fn main() {
    ip_addr_example(); // this is kinda stupid but it gets the wall of text away from here

    let write_message = Message::Write(String::from("hello"));
    write_message.call();

    let _next_message = write_message.next();

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}

fn ip_addr_example() {
    // This is not the way
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Close, but no cigar
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    // Better, but don't reinvent the wheel
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

    // std::net::IpAddr
    // struct Ipv4Addr {
    //     // --snip--
    // }

    // struct Ipv6Addr {
    //     // --snip--
    // }

    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }
}
