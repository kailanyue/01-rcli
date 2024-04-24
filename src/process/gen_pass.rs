use rand::seq::SliceRandom;

// 为了避免相近符号的混淆，移除 O 和 0
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: String,
    lowercase: String,
    number: String,
    symbol: String,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();

    // 保存生成的密码
    let mut password = Vec::new();

    // 保存生成密码的字符，密码会从中随机选择
    let mut chars = Vec::new();

    if uppercase.to_ascii_lowercase().eq("true") {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lowercase.to_ascii_lowercase().eq("true") {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }

    if number.to_ascii_lowercase().eq("true") {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }

    if symbol.to_ascii_lowercase().eq("true") {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    (0..(length - password.len() as u8)).for_each(|_| {
        password.push(*chars.choose(&mut rng).expect("chars won't be empty"));
    });

    // 因为前面部分是固定模式生成的比如第一个永远都是小写...，所以需要对密码进行打乱
    password.shuffle(&mut rng);
    Ok(String::from_utf8(password)?)
}
