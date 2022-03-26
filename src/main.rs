use grep::Config;
use std::{env, process};

fn main() {
    /*  String 元素的 slice 版本
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else：
    //   当 Result 内部为 Err 时，传递 Err 给 err, 然后执行后面 {} 内的闭包，
    //   当 Result 内部为 Ok 时， 返回其值给 config.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Parsing arguments: {}", err);
        process::exit(1); // 表示程序出错退出
    });*/

    // 迭代器版本
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing arguments: {}", err);
        process::exit(1); // 表示程序出错退出
    });

    if let Err(e) = grep::run(config) {
        eprintln!("read file ERROR: {}", e);
        process::exit(1); // 表示程序出错退出
    }
}
