use std::fs::OpenOptions;
use std::io::Write;
use std::error::Error;

/// 将地址和私钥保存到wallet.txt
pub fn save_to_wallet_file(address: &str, private_key: &str, timestamp: &str) -> Result<(), Box<dyn Error>> {
    // 以追加模式打开文件，如果不存在则创建
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("wallet.txt")?;
    
    // 将地址和私钥写入文件
    writeln!(file, "时间戳: {}\n地址: {}\n私钥: {}\n---\n", timestamp, address, private_key)?;
    
    Ok(())
}