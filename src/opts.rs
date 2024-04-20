use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

/// 1.此处的 csv 就是 subcommand 也就是输入的参数
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command()]
    Csv(CsvOpts),
}

/// 1.default_value 和 default_value_t 的区别
/// 2.short 和 long 的区别，以及 header 为什么没有 short

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.csv")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    match Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("Input file not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_is_exist() {
        let result = verify_input_file("assets/juventus.csv").unwrap();
        assert_eq!(result, "assets/juventus.csv");
    }

    #[test]
    fn test_file_is_not_exist() {
        let result = verify_input_file("assets/not_exist.csv").unwrap_err();
        assert_eq!(result, "Input file not found");
    }
}
