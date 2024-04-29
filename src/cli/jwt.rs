use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::Algorithm;

use crate::{process_jwt_sign, process_jwt_verify, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, help = "Audience")]
    pub aud: String,

    #[arg(long, default_value = "14d", help = "Expiration time")]
    pub exp: String,

    #[arg(long, help = "Subject (whom token refers to)")]
    pub sub: String,

    #[arg(long, help = "secret")]
    pub secret: String,

    #[arg(long, value_parser = parse_algorithm_format, default_value = "HS256", help = "token header Algorithm")]
    pub alg: Algorithm,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, help = "token")]
    pub token: String,

    #[arg(long, help = "Audience")]
    pub aud: String,

    #[arg(long, default_value = "14d", help = "Expiration time")]
    pub exp: String,

    #[arg(long, help = "Subject (whom token refers to)")]
    pub sub: String,

    #[arg(long, help = "secret")]
    pub secret: String,

    #[arg(long, value_parser = parse_algorithm_format, default_value = "HS256", help = "token header Algorithm")]
    pub alg: Algorithm,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.aud, self.exp, self.sub, self.secret, self.alg)?;

        println!("token:{}", token);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_jwt_verify(
            self.token.to_string(),
            self.aud.to_owned(),
            self.exp.to_owned(),
            self.sub.to_owned(),
            self.secret.to_owned(),
            self.alg,
        );

        println!("ret: {:?}", ret?);
        Ok(())
    }
}

pub fn parse_algorithm_format(format: &str) -> Result<Algorithm, anyhow::Error> {
    match Algorithm::from_str(format.to_ascii_uppercase().as_str()) {
        Ok(alg) => Ok(alg),
        Err(e) => anyhow::bail!(e),
    }
}
