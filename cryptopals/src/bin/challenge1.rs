use cryptopals::base64::bytes_to_base64;
use cryptopals::hex::hex_str_to_bytes;

const TEST_INPUT_HEX_STR: &str =
    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const TEST_OUTPUT_STR: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let bytes = hex_str_to_bytes(TEST_INPUT_HEX_STR).expect("Could not parse hex string");
    let b64 = bytes_to_base64(bytes.as_slice()).expect("Could not parse base 64");
    if b64 == TEST_OUTPUT_STR {
        println!("It worked!")
    } else {
        println!("Failed!");
        println!("  Expected: {}", TEST_OUTPUT_STR);
        println!("  Actual  : {}", b64);
    }
}
