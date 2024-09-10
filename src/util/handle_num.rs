use std::str::FromStr;

pub fn parse_human_readable_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim().to_lowercase();
    let multiplier = if size_str.ends_with("kb") {
        1024
    } else if size_str.ends_with("mb") {
        1024 * 1024
    } else if size_str.ends_with("gb") {
        1024 * 1024 * 1024
    } else if size_str.ends_with("b") {
        1
    } else {
        return Err(format!("Invalid file size format: {}", size_str));
    };

    let number_str = size_str.trim_end_matches(|c: char| !c.is_digit(10));
    match u64::from_str(number_str) {
        Ok(number) => Ok(number * multiplier),
        Err(_) => Err(format!("Invalid number: {}", number_str)),
    }
}
