use cryptopals::hex::hex_str_to_bytes;

const BUFFER1: &str = "1c0111001f010100061a024b53535009181c";
const BUFFER2: &str = "686974207468652062756c6c277320657965";
const EXPECTED_XOR_OF_BUFFERS: &str = "746865206b696420646f6e277420706c6179";

fn main() {
    let bytes1 = hex_str_to_bytes(BUFFER1).expect("Buffer 1 hex string could not be parsed");
    let bytes2 = hex_str_to_bytes(BUFFER2).expect("Buffer 2 hex string could not be parsed");
    let expected_xor = hex_str_to_bytes(EXPECTED_XOR_OF_BUFFERS)
        .expect("Expected XOR of buffers could not be converted from hex to bytes");
    if bytes1.len() != bytes2.len() {
        println!("Error: The byte buffers are different lengths!");
    } else {
        let xor_bytes = bytes1
            .iter()
            .zip(bytes2.iter())
            .map(|(&byte1, &byte2)| byte1 ^ byte2)
            .collect::<Vec<u8>>();
        assert_eq!(xor_bytes, expected_xor);
        println!("The buffers produce the expected result when XOR'ed together!");
    }
}
