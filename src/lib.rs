use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        /*
        注意 vector 的第一个值 (args[0]) 是 "target/debug/grep"，它是我们二进制文件的名称。
        这与 C 中的参数列表的行为相匹配，让程序使用在执行时调用它们的名称。
        如果要在消息中打印它或者根据用于调用程序的命令行别名更改程序的行为，
        通常可以方便地访问程序名称，不过考虑到本章的目的，我们将忽略它并只保存所需的两个参数。*/
        let query = args[1].clone();
        let filename = args[2].clone();
        /*使用 clone 的权衡取舍
        由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。
        在关于迭代器的第 13 章中，我们将会学习如何更有效率的处理这种情况，
        不过现在，复制一些字符串来取得进展是没有问题的，
        因为只会进行一次这样的拷贝，而且文件名和要搜索的字符串都比较短。
        在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。
        随着你对 Rust 更加熟练，将能更轻松的直奔合适的方法，不过现在调用 clone 是完全可以接受的。
        */
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("\nContents is: \n{}", contents);
    Ok(())
}
