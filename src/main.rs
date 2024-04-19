use clap::Parser;
#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
struct Opts{
    #[command(subcommand)]
    command: SubCommand,
}

/// 1.此处的 csv 就是 subcommand 也就是输入的参数
#[derive(Debug, Parser)]
enum SubCommand {
    #[command()]
    Csv(CsvOpts),
}

/// 1. default_value 和 default_value_t 的区别
/// 2. short 和 long 的区别，以及 header 为什么没有 short

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.csv")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}


/// `cargo run -- csv  -i ./assets/juventus.csv` 测试命令
fn main() {
    let opts = Opts::parse();
    opts.command;
}
