use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ADDRESS_REGEX: Regex = Regex::new(r"^0x[0-9a-fA-F]{40}$").unwrap();
}

/// 用于地址模式匹配的结构体
#[derive(Clone)]
pub struct PatternMatcher {
    prefix_patterns: Vec<Regex>,
    suffix_patterns: Vec<Regex>,
    contains_patterns: Vec<Regex>,
}

impl PatternMatcher {
    /// 使用给定模式创建新的PatternMatcher
    /// 以'^'开头的模式将被视为前缀
    /// 以'$'结尾的模式将被视为后缀
    /// 其他模式将被视为包含匹配
    pub fn new(patterns: Vec<String>) -> Self {
        let mut prefix_patterns = Vec::new();
        let mut suffix_patterns = Vec::new();
        let mut contains_patterns = Vec::new();

        for pattern in patterns {
            let pattern = pattern.to_lowercase();

            if pattern.starts_with('^') {
                // 前缀模式
                let regex = Regex::new(&format!("^{}", &pattern[1..])).unwrap();
                prefix_patterns.push(regex);
            } else if pattern.ends_with('$') {
                // 后缀模式
                let regex = Regex::new(&format!("{}$", &pattern[..pattern.len() - 1])).unwrap();
                suffix_patterns.push(regex);
            } else {
                // 包含模式
                let regex = Regex::new(&pattern).unwrap();
                contains_patterns.push(regex);
            }
        }

        PatternMatcher {
            prefix_patterns,
            suffix_patterns,
            contains_patterns,
        }
    }

    /// 检查地址是否匹配任何模式
    pub fn matches_any(&self, address: &str) -> bool {
        // 验证地址格式
        if !ADDRESS_REGEX.is_match(address) {
            return false;
        }

        // 转换为小写以进行不区分大小写的匹配
        let address = address.to_lowercase();
        let address_without_prefix = &address[2..]; // 移除"0x"前缀

        // 检查前缀模式
        if self
            .prefix_patterns
            .iter()
            .any(|re| re.is_match(address_without_prefix))
        {
            return true;
        }

        // 检查后缀模式
        if self
            .suffix_patterns
            .iter()
            .any(|re| re.is_match(address_without_prefix))
        {
            return true;
        }

        // 检查包含模式
        if self
            .contains_patterns
            .iter()
            .any(|re| re.is_match(address_without_prefix))
        {
            return true;
        }

        false
    }
}
