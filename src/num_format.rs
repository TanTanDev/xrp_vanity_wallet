/// pretty format a number to string
/// example: 12345678 = 12'345'678
pub fn format_with_separator(n: i32) -> String {
    let mut chars: Vec<char> = n.to_string().chars().rev().collect();
    for i in (3..chars.len()).step_by(3) {
        chars.insert(i, '\'');
    }
    chars.into_iter().rev().collect()
}
