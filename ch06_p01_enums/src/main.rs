enum IpAddrKind { 
    V4,
    V6,
}

enum IpAddr {
    // we can indicate that both variants
    // have associated String values
    V4(String), 
    V6(String),
}

enum IpAddr2 {
    // different variants can have 
    // different associated values
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling...");
    }
}

fn main() {

    // the underscore at the beginning of variable names
    // indicates that the variable is not used.
    // so that rustc wont throw a warning

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // each enum variant also becomes a function that constructs
    // an instance of the enum
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let _local = IpAddr2::V4(127, 0, 0, 1);
    let _loopb = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _q = Message::Quit;
    let _move = Message::Move { x: 4, y: 2 };
    let _color = Message::ChangeColor(4, 2, 1);

    // Option<T> { None, Some(T)}
    let _some_number = Some(34);
}
