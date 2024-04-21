use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
    }
    Ok(())
}

/*
git tag -a v1.7-csv
git push -u origin v1.7-csv


git tag -d v1.7-csv
git push origin --delete v1.7-csv
*/
