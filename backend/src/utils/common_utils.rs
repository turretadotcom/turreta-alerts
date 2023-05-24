pub fn public_key_normalize_format(public_key: String) -> String {
    let chunked_key = public_key
        .chars()
        .collect::<Vec<char>>()
        .chunks(64)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    format!("-----BEGIN PUBLIC KEY-----\n{0}\n-----END PUBLIC KEY-----\n", chunked_key)
}