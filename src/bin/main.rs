use leto;
fn main() {
    let input = String::from("Hello, World! 한 🤢😎💋🐱‍🏍");
    let x:Vec<char> = leto::byte2char::Byte2Char::new(input.as_bytes()).collect();
    let y:Vec<char> = input.chars().collect();
    println!("{:?}", x);
    println!("{:?}", y);
}