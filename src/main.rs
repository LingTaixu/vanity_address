use chrono::Local;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;

mod generator;
mod patterns;
mod storage;

use generator::generate_eth_address;
use patterns::PatternMatcher;
use storage::save_to_wallet_file;

fn main() {
    // 配置 - 此处可以修改为命令行参数
    let num_threads: usize = num_cpus::get(); // 使用所有可用CPU
    let patterns = vec![
        String::from("00ff00ff00ff00"),
        String::from("ff00ff00ff00"),
        String::from("bbddbbddbbdd"),
        String::from("cccccccccccc"),
        String::from("aaaaaaaaaaaa"),
        String::from("aaaaaaaa$"),
        String::from("bbbbbbbb$"),
        String::from("cccccccc$"),
        String::from("dddddddd$"),
        String::from("eeeeeeee$"),
        String::from("ffffffff$"),
        String::from("00000000$"),
        String::from("11111111$"),
        String::from("22222222$"),
        String::from("33333333$"),
        String::from("44444444$"),
        String::from("55555555$"),
        String::from("66666666$"),
        String::from("77777777$"),
        String::from("88888888$"),
        String::from("99999999$"),
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

    // 添加进度显示线程
    let progress_generated = Arc::clone(&total_generated);
    let progress_matches = Arc::clone(&total_matches);
    let start_time = Instant::now();
    let prev_count = Arc::new(Mutex::new(0u64));
    let prev_time = Arc::new(Mutex::new(start_time));

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5)); // 每5秒更新一次进度

            let current_count = *progress_generated.lock().unwrap();
            let current_matches = *progress_matches.lock().unwrap();
            let current_time = Instant::now();

            let mut prev_count_guard = prev_count.lock().unwrap();
            let mut prev_time_guard = prev_time.lock().unwrap();

            let elapsed = current_time.duration_since(*prev_time_guard).as_secs_f64();
            let count_diff = current_count - *prev_count_guard;
            let speed = count_diff as f64 / elapsed;

            *prev_count_guard = current_count;
            *prev_time_guard = current_time;

            let total_elapsed = current_time.duration_since(start_time).as_secs();
            let hours = total_elapsed / 3600;
            let minutes = (total_elapsed % 3600) / 60;
            let seconds = total_elapsed % 60;

            println!(
                "[进度] 已生成: {} 地址 | 匹配: {} 地址 | 速度: {:.2} 地址/秒 | 运行时间: {}h{}m{}s",
                current_count,
                current_matches,
                speed,
                hours,
                minutes,
                seconds
            );
        }
    });

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

    // 等待所有任务完成
    pool.join();

    // 显示最终统计信息
    let final_generated = *total_generated.lock().unwrap();
    let final_matches = *total_matches.lock().unwrap();
    let final_time = start_time.elapsed().as_secs();

    println!("\n=== 生成完成 ===");
    println!("总生成地址数: {}", final_generated);
    println!("匹配地址数: {}", final_matches);
    println!(
        "总运行时间: {}h{}m{}s",
        final_time / 3600,
        (final_time % 3600) / 60,
        final_time % 60
    );
    if final_matches > 0 {
        println!(
            "匹配率: {:.8}%",
            (final_matches as f64 / final_generated as f64) * 100.0
        );
    }
}
