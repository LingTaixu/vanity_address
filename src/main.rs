use chrono::Local;
use num_cpus;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

mod generator;
mod patterns;
mod storage;

use generator::generate_eth_address;
use patterns::PatternMatcher;
use storage::save_to_wallet_file;

fn main() {
    // 配置 - 此处可以修改为命令行参数
    let num_threads: usize = 10; // 使用所有可用CPU
    let patterns = vec![
        String::from("00ff00ff00ff00"),
        String::from("ff00ff00ff00"),
        String::from("bbddbbddbbdd"),
        String::from("cccccccccccc"),
        String::from("aaaaaaaaaaaa"),
    ];

    println!("使用 {} 线程启动以太坊漂亮地址生成器", num_threads);
    println!("搜索匹配模式: {:?}", patterns);
    println!("结果将保存到 wallet.txt");

    // 创建模式匹配器
    let matcher = Arc::new(Mutex::new(PatternMatcher::new(patterns)));
    let n_jobs: u64 = 888_888_888_888_888;
    // 创建线程池
    let pool: ThreadPool = ThreadPool::new(num_threads);

    // 跟踪生成的地址数量
    let total_generated = Arc::new(Mutex::new(0u64));
    let total_matches = Arc::new(Mutex::new(0u64));

    for _ in 0..n_jobs {
        let matcher = Arc::clone(&matcher);
        let total_generated = Arc::clone(&total_generated);
        let total_matches = Arc::clone(&total_matches);
        if pool.queued_count() <= num_threads {
            pool.execute(move || {
                // 生成新的以太坊地址
                let (address, private_key) = generate_eth_address();

                // 更新生成总数
                let mut guard = total_generated.lock().unwrap();
                *guard += 1;

                // 检查地址是否匹配任何模式
                let matcher = matcher.lock().unwrap();
                if matcher.matches_any(&address) {
                    // 更新匹配计数
                    *total_matches.lock().unwrap() += 1;

                    // 保存到文件
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    save_to_wallet_file(&address, &private_key, &timestamp).unwrap();

                    // 打印匹配信息
                    println!("[{}] 找到匹配地址: {}", timestamp, address);
                }
            });
        }
    }
}
