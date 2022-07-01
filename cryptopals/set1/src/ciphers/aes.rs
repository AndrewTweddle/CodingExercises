use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;

pub fn decrypt_aes_128_ecb(key: &[u8; 16], encrypted_bytes: &[u8]) -> Vec<u8> {
    let aes_cipher = Cipher::aes_128_ecb();
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(aes_cipher), Some(key), None).unwrap();

    let mut plain_text_bytes: Vec<u8> = vec![];
    ctx.cipher_update_vec(encrypted_bytes, &mut plain_text_bytes).unwrap();
    ctx.cipher_final_vec(&mut plain_text_bytes).unwrap();
    plain_text_bytes
}
