use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;
use openssl::error::ErrorStack;

pub fn decrypt_aes_128_ecb(key: &[u8; 16], encrypted_bytes: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let aes_cipher = Cipher::aes_128_ecb();
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(aes_cipher), Some(key), None)?;

    let mut plain_text_bytes: Vec<u8> = vec![];
    ctx.cipher_update_vec(encrypted_bytes, &mut plain_text_bytes)?;
    ctx.cipher_final_vec(&mut plain_text_bytes)?;
    Ok(plain_text_bytes)
}
