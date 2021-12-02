use set1::hex::hex_str_to_bytes;

const BUFFER1: &str = "1c0111001f010100061a024b53535009181c";
const BUFFER2: &str = "686974207468652062756c6c277320657965";
const EXPECTED_XOR_OF_BUFFERS: &str = "746865206b696420646f6e277420706c6179";

fn main() {
    let bytes1 = hex_str_to_bytes(BUFFER1);
    let bytes2 = hex_str_to_bytes(BUFFER2);
    let expected_xor = hex_str_to_bytes(EXPECTED_XOR_OF_BUFFERS);
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