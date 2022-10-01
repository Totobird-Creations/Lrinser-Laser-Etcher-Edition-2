pub fn commaify_i64(number : i64) -> String {
    let     string = number.abs().to_string();
    let mut result = String::new();
    let mut chars  = string.chars().collect::<Vec<char>>();
    chars.reverse();
    for i in 0..chars.len() {
        if (i != 0 && i % 3 == 0) {
            result += "_";
        }
        result += chars.get(i).unwrap().to_string().as_str();
    }
    return format!(
        "{}{}",
        if (number < 0) {"-"} else {""},
        result.chars().rev().collect::<String>()
    );
}