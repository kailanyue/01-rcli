use clap::Parser;

use crate::{process_genpass, CmdExector};
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, help = "password length")]
    pub length: u8,

    #[arg(long, default_value = "true", help = "include uppercase letters")]
    pub uppercase: String,

    #[arg(long, default_value = "true", help = "include lowercase letters")]
    pub lowercase: String,

    #[arg(long, default_value = "false", help = "include numbers")]
    pub number: String,

    #[arg(long, default_value = "false", help = "include symbols")]
    pub symbol: String,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);

        // output password strength in stderr
        let estimate = zxcvbn(&ret, &[]);
        eprintln!("Password strength: {}", estimate.score());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genpass() {
        let opts = GenPassOpts {
            length: 16,
            uppercase: "true".to_string(),
            lowercase: "true".to_string(),
            number: "false".to_string(),
            symbol: "false".to_string(),
        };
        let ret = process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )
        .unwrap();
        println!("{}", ret);
    }
}
