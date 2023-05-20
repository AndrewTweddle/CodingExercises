use cryptopals::ciphers::repeating_key_xor::encrypt;
use cryptopals::hex::bytes_to_hex_str;

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble\n\
                 I go crazy when I hear a cymbal";
    let key = "ICE";
    let encrypted_bytes = encrypt(input, key);
    let output = bytes_to_hex_str(&encrypted_bytes);

    let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3\
                           c6324202d623d63343c2a2622632427276527\
                           2a282b2f20430a652e2c652a3124333a653e2\
                           b2027630c692b20283165286326302e27282f";
    assert_eq!(
        output, expected_output,
        "encrypted output should equal expected output"
    );

    println!("Encrypted text in hex: {}", output);
}
