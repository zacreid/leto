use leto;
fn main() {
    let input = String::from("Hello, World! 한 🤢 😎 💋 🐱‍🏍");
    let x:Vec<char> = leto::byte2char::Byte2Char::new(input.as_bytes()).collect();
    let y:Vec<char> = input.chars().collect();
    let mut c:std::cell::Cell<String> = std::cell::Cell::default();
    let mut z = leto::char2token::Char2Token::new(leto::byte2char::Byte2Char::new(input.as_bytes()), &c).next();
    //let a = leto::char2token::Char2Token::from(input.chars()).last();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", c.get_mut());
    //println!("{:?}", a);
}