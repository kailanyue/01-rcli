use std::fmt;

use anyhow::Result;
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: u64,
    sub: String,
}

impl Claims {
    pub fn new(aud: String, exp: u64, sub: String) -> Self {
        Self { aud, exp, sub }
    }

    pub fn try_new(aud: String, exp: String, sub: String) -> Result<Self> {
        let exp_ts = OffsetDateTime::now_utc() + humantime::parse_duration(&exp)?;
        Ok(Self::new(aud, exp_ts.unix_timestamp() as u64, sub))
    }
}

impl fmt::Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "claims(aud:{}, exp:{}, sub:{})",
            self.aud, self.exp, self.sub
        )
    }
}

pub fn process_jwt_sign(
    aud: String,
    exp: String,
    sub: String,
    secret: String,
    alg: Algorithm,
) -> Result<String> {
    let header = Header {
        alg,
        ..Default::default()
    };

    let claims = Claims::try_new(aud, exp, sub)?;

    let token = match jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(e) => anyhow::bail!("in practice you would return the error: {}", e), //
    };
    println!("claims:{}", claims);

    Ok(token)
}

pub fn process_jwt_verify(
    token: String,
    aud: String,
    exp: String,
    sub: String,
    secret: String,
    alg: Algorithm,
) -> Result<bool> {
    let mut validation = Validation::new(alg);
    let claims = Claims::try_new(aud, exp, sub)?;

    validation.sub = Some(claims.sub);
    validation.set_audience(&[claims.aud]);
    validation.set_required_spec_claims(&["exp"]);

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    );

    match token_data {
        Ok(c) => {
            println!("{}", c.claims);
            println!("{:?}", c.header);
            Ok(true)
        }
        Err(err) if *err.kind() == ErrorKind::InvalidToken => panic!("Token is invalid"),
        Err(err) if *err.kind() == ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
        _ => Err(anyhow::anyhow!("Some other errors")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, encode, Algorithm};

    // cargo run jwt sign --aud me --sub test --exp 10d --key secret

    #[test]
    fn test_process_jwt_sign() -> Result<()> {
        let token = process_jwt_sign(
            "test_aud1".to_owned(),
            "10d".to_owned(),
            "test_sub1".to_owned(),
            "QkVUKy_r1V#Ht7D8S".to_owned(),
            Algorithm::HS512,
        );

        assert!(token.is_ok());
        println!("token: {}", token.unwrap());

        Ok(())
    }

    // claims:claims(aud:test_aud1, exp:1715266953, sub:test_sub1)
    #[test]
    fn test_process_jwt_verify_ok() -> Result<()> {
        let token = process_jwt_sign(
            "test_aud1".to_owned(),
            "10d".to_owned(),
            "test_sub1".to_owned(),
            "QkVUKy_r1V#Ht7D8S".to_owned(),
            Algorithm::HS512,
        )?;

        let ret = process_jwt_verify(
            token.to_string(),
            "test_aud1".to_owned(),
            "10d".to_owned(),
            "test_sub1".to_owned(),
            "QkVUKy_r1V#Ht7D8S".to_owned(),
            Algorithm::HS512,
        );
        assert!(ret.is_ok());
        Ok(())
    }

    #[test]
    fn test_process_jwt_verify_error() -> Result<()> {
        let token = process_jwt_sign(
            "test_aud1".to_owned(),
            "10d".to_owned(),
            "test_sub1".to_owned(),
            "QkVUKy_r1V#Ht7D8S".to_owned(),
            Algorithm::HS512,
        )?;

        let token_invalid = token.to_string() + "invalid";

        let ret = process_jwt_verify(
            token_invalid,
            "test_aud1".to_owned(),
            "10d".to_owned(),
            "test_sub1".to_owned(),
            "QkVUKy_r1V#Ht7D8S".to_owned(),
            Algorithm::HS512,
        );
        assert!(ret.is_err());
        Ok(())
    }

    #[test]
    fn test_process_jwt_sign_verify() {
        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let now = OffsetDateTime::now_utc();
        let t1 = now + humantime::parse_duration("14d").unwrap();

        let key = b"secret";
        let my_claims = Claims {
            aud: "aud_ngt".to_owned(),
            sub: "b@b.com".to_owned(),
            exp: t1.unix_timestamp() as u64,
        };

        let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };

        println!("token: {}", token);

        let mut validation = Validation::new(Algorithm::HS512);
        validation.sub = Some("b@b.com".to_string());
        validation.set_audience(&["aud_ngt"]);
        validation.set_required_spec_claims(&["exp"]);

        println!("validation: {:?}", validation);

        let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation)
        {
            Ok(c) => c,
            Err(err) if *err.kind() == ErrorKind::InvalidToken => panic!("Token is invalid"),
            Err(err) if *err.kind() == ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        };

        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);
    }
}
