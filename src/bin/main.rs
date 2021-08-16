use leto;
fn main() {
    let input = String::from("Hello, World! ❤️");
    let x = leto::byte2char::Byte2Char::new(input.as_bytes()).last();
    println!("{:?}", x);
}