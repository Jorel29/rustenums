
//declaring an enum
// is now like a data type
enum IpAddrKind{
    V4,
    V6,
}

//an enum like this is like defining different struct definitions
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}
static _home = IpAddr::V4(127,0,0,1);

let _loopback = IpAddr::V6(String::from("::1")); 

fn route(ip_kind: IpAddrKind){

}
// you can define different fields in an enum
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor(i32, i32, i32),
}
// you can also define functions on enums
impl Message{
    fn call(&self){
        //method body
    }
}

let m = Message::Write(String::from("hello"));

m.call();

//There is an option<T> None enum for a non-valid value

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}
