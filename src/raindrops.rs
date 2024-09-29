fn raindrop(input:u32) -> String {
    let mut result = String::new();
    if input % 3 == 0 {
        result.push_str("Pling");
    }
    if input  % 5 == 0 {
    result.push_str("Plang");
    }
    if input % 7 == 0 {
    result.push_str("Plong");
    }
    if result.is_empty() {
    result = input.to_string();
    }
    result
}
