use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

/// `cargo run -- csv  -i ./assets/juventus.csv` 测试命令
/// `cargo run -- csv  -i ./assets/juventus.csv -o ./assets/juventus.json` 测试命令

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}

/*
git tag -a v1-7-csv
git push -u origin v1-7-csv


git tag -d v1-7-csv
git push origin --delete v1-7-csv
*/
