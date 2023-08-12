#![allow(dead_code, unused)]
use std::env;

use clap::Parser;

/// 環境変数PATHを一覧表示するコマンドです
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // / Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // / Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    /// 指定すると含まれる変数値だけを表示します
    // filter: Option<String>
    filter: Option<String>,
}

fn main() {
    let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    // print_path();

    // println!("{:?}", args.filter.as_deref());

    // match args.filter {
    //     Some(v) => print_path(),
    //     None => {}
    // }
    let a = args.filter.as_deref();
    print_path(a)
}

fn match_str(str: &str, find: &str) -> bool {
    str.to_lowercase().contains(&find.to_lowercase())
}

fn print_path(filter: Option<&str>) {
    let pathstr = env::var("PATH").unwrap();
    // let pathstr = env::var("PATH").expect("");
    // println!("env:{}", pathstr);

    for path_line in pathstr.split(";") {
        if path_line.trim() == "" {
            continue;
        }
        if filter.map_or(true, |x| match_str(path_line, x)) {
            println!("{}", path_line);
        }
    }
}

#[test]
fn test() {
    let x: Option<String> = Some("".into());
    assert!(x.map_or(true, |x| x == ""));
    let x: Option<String> = None;
    assert!(x.map_or(true, |x| x == ""));
}
