 
enum IpAddrKind {
    V4, 
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {
    // println!("{}", ip_kind);
}
enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// ------------

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// ------------


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32, 
    y: i32,
}
struct WriteMessage(String);  // tuple struct
struct ChangeColorMessage(i32, i32, i32 ); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
        // println!("Message call(): {}", self::Quit);
    }
}

// ---------------------

enum Option2<T> {
    None, 
    Some(T),
}

// --------------------

fn main() {
    let four = IpAddrKind::V4; 
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // -----------------------

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));
  
    // -----------------------

    let home3 = IpAddr3::V4(127, 0, 0,1);
    let loopback3 = IpAddr3::V6(String::from("::1"));
    
    // -----------------------

    let home4 = IpAddr4::V4(Ipv4Addr{});
    let loopback4 = IpAddr4::V6(Ipv6Addr{});

    // -----------------------
    let m = Message::Write(String::from("hello"));
    m.call();

    // -----------------------
    let some_number = Option2::Some(5);
    let some_string = Option2::Some("a string");

    let absent_number: Option2<i32> = Option2::None;

    let x: i8 = 5; 
    let y: Option2<i8> = Option2::Some(5);
    
    let sum = x + y; // error: no implementation for `i8 + Option2<i8>`
}
