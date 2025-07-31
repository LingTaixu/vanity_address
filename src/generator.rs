use hex::encode;
use rand::rngs::OsRng;
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};

/// 生成新的以太坊地址和对应的私钥
pub fn generate_eth_address() -> (String, String) {
    // 生成随机secp256k1私钥
    let mut rng = OsRng;

    let secret_key = SecretKey::new(&mut rng);

    // 从私钥派生公钥
    let public_key = PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &secret_key);

    // 将公钥转换为以太坊地址
    let public_key_uncompressed = public_key.serialize_uncompressed();
    // 注意：需要先执行 `cargo add tiny_keccak` 将 tiny_keccak 依赖添加到 Cargo.toml 中
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&public_key_uncompressed[1..]);
    keccak.finalize(&mut output);
    let address_bytes = &output[12..];

    // 返回地址和私钥的十六进制字符串
    (
        format!("0x{}", encode(address_bytes)),
        format!("0x{}", encode(secret_key.secret_bytes())),
    )
}
