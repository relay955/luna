use sha2::{Digest, Sha256};

pub fn str_to_sha256_binary(str: &str) -> Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();
    result.to_vec()
}
pub fn str_to_sha256_hex(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();
    let result_str = format!("{:x}", result);
    return result_str;
}
