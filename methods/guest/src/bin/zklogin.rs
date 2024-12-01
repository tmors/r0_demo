#![no_main]
// #![no_std]
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
// 私密输入 a (要搜索的主字符串)

    println!("this is guest zklogin");
    let text_a: String = env::read();
    // 公共输入 b (要查找的子字符串)
    let pattern_b: String = env::read();

    // 进行包含性检查
    let contains = text_a.contains(&pattern_b);

    println!("{}/{}/{}", text_a, pattern_b, contains);

    // 提交公共输入和结果
    // 注意：我们提交 pattern_b 使其成为公开输入
    env::commit(&(pattern_b, contains));
}
