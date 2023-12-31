use base64_simd::STANDARD;
use bytes::Bytes;
use const_decoder::Decoder;
use rsa::{pkcs8::DecodePrivateKey, Pkcs1v15Encrypt, RsaPrivateKey};
use snafu::ResultExt;

use std::sync::OnceLock;

use crate::error::{DecryptSnafu, DmzjResult};

const PRIVATE_KEY_BASE64: &[u8] = b"MIICeAIBADANBgkqhkiG9w0BAQEFAASCAmIwggJeAgEAAoGBAK8nNR1lTnIfIes6oRWJNj3mB6OssDGx0uGMpgpbVCpf6+VwnuI2stmhZNoQcM417Iz7WqlPzbUmu9R4dEKmLGEEqOhOdVaeh9Xk2IPPjqIu5TbkLZRxkY3dJM1htbz57d/roesJLkZXqssfG5EJauNc+RcABTfLb4IiFjSMlTsnAgMBAAECgYEAiz/pi2hKOJKlvcTL4jpHJGjn8+lL3wZX+LeAHkXDoTjHa47g0knYYQteCbv+YwMeAGupBWiLy5RyyhXFoGNKbbnvftMYK56hH+iqxjtDLnjSDKWnhcB7089sNKaEM9Ilil6uxWMrMMBH9v2PLdYsqMBHqPutKu/SigeGPeiB7VECQQDizVlNv67go99QAIv2n/ga4e0wLizVuaNBXE88AdOnaZ0LOTeniVEqvPtgUk63zbjl0P/pzQzyjitwe6HoCAIpAkEAxbOtnCm1uKEp5HsNaXEJTwE7WQf7PrLD4+BpGtNKkgja6f6F4ld4QZ2TQ6qvsCizSGJrjOpNdjVGJ7bgYMcczwJBALvJWPLmDi7ToFfGTB0EsNHZVKE66kZ/8Stx+ezueke4S556XplqOflQBjbnj2PigwBN/0afT+QZUOBOjWzoDJkCQClzo+oDQMvGVs9GEajS/32mJ3hiWQZrWvEzgzYRqSf3XVcEe7PaXSd8z3y3lACeeACsShqQoc8wGlaHXIJOHTcCQQCZw5127ZGs8ZDTSrogrH73Kw/HvX55wGAeirKYcv28eauveCG7iyFR0PFB/P/EDZnyb+ifvyEFlucPUI0+Y87F";

const PRIVATE_KEY_BYTES: [u8; 636] = Decoder::Base64.decode(PRIVATE_KEY_BASE64);

const MAX_DECRYPT_BLOCK: usize = 128;

fn get_private_key() -> &'static RsaPrivateKey {
    static RSA_PRIVATE_KEY: OnceLock<RsaPrivateKey> = OnceLock::new();

    RSA_PRIVATE_KEY.get_or_init(|| {
        RsaPrivateKey::from_pkcs8_der(&PRIVATE_KEY_BYTES).unwrap()
    })
}

fn decrypt(encrypted: Bytes, key: &RsaPrivateKey) -> DmzjResult<Vec<u8>> {
    let mut result = Vec::with_capacity(1024);
    let encrypted_data = STANDARD.decode_to_vec(encrypted).unwrap();

    for chunk in encrypted_data.chunks(MAX_DECRYPT_BLOCK) {
        let dec = key.decrypt(Pkcs1v15Encrypt, chunk).context(DecryptSnafu)?;
        result.extend_from_slice(&dec);
    }

    Ok(result)
}

pub fn decrypt_bytes(b: Bytes) -> DmzjResult<Vec<u8>> {
    let key = get_private_key();

    decrypt(b, key)
}
