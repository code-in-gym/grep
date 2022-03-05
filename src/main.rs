use grep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else：
    //   当 Result 内部为 Err 时，传递 Err 给 err, 然后执行后面 {} 内的闭包，
    //   当 Result 内部为 Ok 时， 返回其值给 config.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Parsing arguments: {}", err);
        process::exit(1); // 表示程序出错退出
    });

    if let Err(e) = grep::run(config) {
        eprintln!("read file ERROR: {}", e);
        process::exit(1); // 表示程序出错退出
    }
}

// Usage:
// IS_SENSITIVE=1 cargo run us poem.txt
// or
// cargo run us poem.txt
