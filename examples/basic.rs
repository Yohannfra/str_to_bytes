use str_to_bytes::str_to_bytes;

fn main() {
    let s: String = "12 0xff 0x25 0b1011".to_owned();

    let bytes: Vec<u8> = str_to_bytes(&s).unwrap();

    println!("{:?}", bytes);
}
