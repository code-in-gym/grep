use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
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

        // env::var 返回一个 Result，
        // 它在环境变量被设置时返回包含其值的 Ok 成员，
        // 并在环境变量未被设置时返回 Err 成员。
        let is_sensitive = env::var("IS_SENSITIVE").is_ok();
        Ok(Config { query, filename, is_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("Result of search is:");
    let res = if config.is_sensitive {
        search_sensitive(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    for v in res {
        println!("{}", v)
    }
    Ok(())
    
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_4_new_config_1() -> Result<(), String> {
        let mut args: Vec<String> = vec!["".to_string()];
        args.push("query".to_string());
        args.push("filename".to_string());
        let config = Config::new(&args)?;
        assert_eq!(&config.query, &args[1]);
        assert_eq!(&config.filename, &args[2]);
        Ok(())
    }

    #[test]
    fn test_4_new_config_2() -> Result<(), String> {
        let mut args: Vec<String> = vec!["".to_string()];
        args.push("query".to_string());
        args.push("filename".to_string());
        args.push("xxx".to_string());
        let config = Config::new(&args)?;
        assert_eq!(&config.query, &args[1]);
        assert_eq!(&config.filename, &args[2]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_4_new_config_err() {
        let mut args: Vec<String> = vec!["".to_string()];
        args.push("query".to_string());
        let _res = Config::new(&args);
        let _config = match _res {
            Ok(config) => config,
            Err(err_str) => {
                panic!("{}", err_str)
            }
        };
    }

    #[test]
    #[should_panic]
    fn test_4_new_config_err_2() {
        let args: Vec<String> = vec!["".to_string()];
        let _res = Config::new(&args);
        let _config = match _res {
            Ok(config) => config,
            Err(err_str) => {
                panic!("{}", err_str)
            }
        };
    }
}

#[cfg(test)]
mod run_tests {
    use super::*;

    #[test]
    fn test_run() -> Result<(), String> {
        let mut args: Vec<String> = vec!["".to_string()];
        args.push("query".to_string());
        args.push("poem.txt".to_string());
        let config = Config::new(&args)?;

        if let Err(e) = run(config) {
            assert_eq!("".to_string(), e.to_string())
        }
        Ok(())
    }

    #[test]
    fn test_run_err() -> Result<(), String> {
        let mut args: Vec<String> = vec!["".to_string()];
        args.push("query".to_string());
        args.push("poemNotFound.txt".to_string());
        let config = Config::new(&args)?;

        if let Err(e) = run(config) {
            assert_ne!("".to_string(), e.to_string())
        }
        Ok(())
    }
}

#[cfg(test)]
mod search_test {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "us";
        let contents = "\
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.

        How dreary to be somebody!
        How public, like a frog
        To tell your name the livelong day
        To an admiring bog!";
        assert_eq!(
            vec![
                "        Then there's a pair of us - don't tell!",
                "        They'd banish us, you know.",
            ],
            search_sensitive(query, contents)
        )
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "us";
        let contents = "US
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.

        How dreary to be somebody!
        How public, like a frog
        To tell your name the livelong day
        To an admiring bog!";
        assert_eq!(
            vec![
                "US",
                "        Then there's a pair of us - don't tell!",
                "        They'd banish us, you know.",
            ],
            search_insensitive(query, contents)
        )
    }
}
