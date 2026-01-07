pub fn series(digits: &str, len: usize) -> Vec<String> {
    //todo!("What are the series of length {len} in string {digits:?}")
    if len == 0 || len > digits.len() {
        return vec![];
    }    let mut result = Vec::new();
    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }
    result
}
