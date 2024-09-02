use crate::module::hash;
use aes::Aes256;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::generic_array::GenericArray;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use rand::RngCore;

type AesCbcEnc = Encryptor<Aes256>;
type AesCbcDec = Decryptor<Aes256>;

/// 랜덤 IV를 생성합니다.
fn generate_iv() -> Vec<u8>{
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    iv.to_vec()
}

///iv가 포함된 바이너리를 키로 복호화합니다. iv는 바이너리의 마지막 16바이트에 위치한다고 가정합니다.
pub fn decrypt_binary_with_iv(key:&str, binary: &mut Vec<u8>){
    let key = hash::str_to_sha256_binary(key);
    let iv = binary[binary.len()-16..].to_vec();
    
    let key_array:[u8; 32] = key.try_into().unwrap();
    let iv_array:[u8; 16] = iv.try_into().unwrap();
    
    let key_array = GenericArray::from_slice(&key_array);
    let iv_array = GenericArray::from_slice(&iv_array);

    binary.truncate(binary.len()-16);
    let decryptor = AesCbcDec::new(key_array, iv_array);
    decryptor.decrypt_padded_mut::<Pkcs7>(binary).unwrap();
    //복호화 후 패딩길이만큼 제거
    let padding_len = binary[binary.len()-1];
    binary.truncate(binary.len()-padding_len as usize);
}

///랜덤 iv를 생성하여 바이너리를 암호화합니다. iv는 암호화된 바이너리의 마지막에 추가됩니다.
pub fn encrypt_binary_with_iv(key:&str, binary:&mut Vec<u8>){
    let key = hash::str_to_sha256_binary(key);
    let iv = generate_iv();

    let key_array:[u8; 32] = key.try_into().unwrap();
    let iv_array:[u8; 16] = iv.clone().try_into().unwrap();

    let key_array = GenericArray::from_slice(&key_array);
    let iv_array = GenericArray::from_slice(&iv_array);
    
    let binary_len = binary.len();
    //바이너리 버퍼의 길이가 부족한경우 패딩 추가
    let mut padding_len = 16 - binary_len % 16;
    if padding_len == 0 { padding_len = 16 }
    binary.extend_from_slice(&vec![0u8; padding_len]);
    
    let mut encryptor = AesCbcEnc::new(key_array,iv_array);
    encryptor.encrypt_padded_mut::<Pkcs7>(binary,binary_len).unwrap();
    binary.extend_from_slice(&iv);
}