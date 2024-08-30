pub fn generate_hex_string(length:u32) -> String{
    let mut result = String::new();
    for _ in 0..length{
        let random = rand::random::<u8>();
        result.push_str(&format!("{:x}",random));
    }
    result
}