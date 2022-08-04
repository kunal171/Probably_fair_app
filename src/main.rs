use bs58;
use md5;

fn convert_hex_to_str(bytes: Vec<u8>) -> String {
    println!("MD5 Hash Len: {}", bytes.len());
    let string: Vec<String> = bytes.iter().map(|b| format!("{:x}", b)).collect();
    println!("MD5 Hash: {}", string.join(""));
    let bs58_vec = bs58::encode(string.join("").as_bytes()).into_vec();
    println!("Base-58 Len: {}", bs58_vec.len());
    println!(
        "Base-58: {}",
        bs58::encode(string.join("").as_bytes()).into_string()
    );
    bs58::encode(string.join("")).into_string()
}

fn random_number(client_seed: &str, server_seed: &str, nonce: u64) {
    let hash = md5::compute(format!("{}{}{}", client_seed, server_seed, nonce).into_bytes());
    convert_hex_to_str(hash.to_vec());
}

fn main() {
    random_number(
        "D16AqBiZW1Q6hzbpNL1TjD8gGVTpub4fyADHYTv6qc77",
        "DpUporWQfeCHr7DKjrDTYPvrjk3Y1WSfMQ35CFkWwGG3",
        0
    );
}
