use base16_rs::base16::Encoder;
use base16_rs::base16::Type::StdEncoding;

fn main() {
    println!("hello base16");
    base16_rs::add(120, 140);

    let str = String::from("abcdefghijklmnop");

    let buf: Vec<u8> = str.into_bytes();

    let encoder = Encoder::new(StdEncoding);
    let result  = encoder.encode_upper_to_string(&buf);
    println!("{:?}", result);
}
