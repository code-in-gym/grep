use std::{env, fs};

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    /*
    注意 vector 的第一个值是 "target/debug/grep"，它是我们二进制文件的名称。
    这与 C 中的参数列表的行为相匹配，让程序使用在执行时调用它们的名称。
    如果要在消息中打印它或者根据用于调用程序的命令行别名更改程序的行为，
    通常可以方便地访问程序名称，不过考虑到本章的目的，我们将忽略它并只保存所需的两个参数。*/
   let query = &args[1];
   let filename = &args[2];

   println!("Search: {}", query);
   println!("in file: {}", filename);

   let contents = fs::read_to_string(filename).expect("Open file ERROR:");
   println!("\nContents is: \n{}", contents)
}
