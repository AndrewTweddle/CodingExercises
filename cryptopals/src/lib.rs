pub mod base64;
pub mod ciphers;
pub mod hex;

pub fn xor_bytes_with_key(bytes: &[u8], key: u8) -> Vec<u8> {
    bytes.iter().map(|&byte| byte ^ key).collect::<Vec<u8>>()
}
