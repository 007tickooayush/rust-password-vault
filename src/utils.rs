pub fn println(token: &str, prefix: Option<&str>) {
    if let Some(prefix) = prefix {
        println!("{prefix}{token}");
    } else {
        println!("{token}");
    }
}