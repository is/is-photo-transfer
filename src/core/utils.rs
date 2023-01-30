pub fn env_var(key: &str) -> Option<String> {
    std::env::var_os(key).map(|e| e.into_string().unwrap())
}
