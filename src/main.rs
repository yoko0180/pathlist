use clap::Parser;

/// 環境変数PATHを一覧表示するコマンドです
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 指定すると含まれる変数値だけを表示します
    #[arg(short, long)]
    filter: Option<String>,
}

fn main() {
    let args = Args::parse();
    let a = args.filter.as_deref();
    print_path(a)
}

fn match_str(str: &str, find: &str) -> bool {
    str.to_lowercase().contains(&find.to_lowercase())
}

fn print_path(filter: Option<&str>) {
    let pathstr = std::env::var("PATH").unwrap();
    for path_line in pathstr.split(";") {
        if path_line.trim() == "" {
            continue;
        }
        if filter.map_or(true, |x| match_str(path_line, x)) {
            println!("{}", path_line);
        }
    }
}
