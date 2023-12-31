pub fn get_config_directory() -> String {
    let mut path = std::env::current_dir().expect("Failed to get current directory");

    // If executed from target (e.g., tests), move up one directory
    if path.ends_with("server") {
        path.pop();
    }

    path.push("config");

    path.to_str()
        .expect("Failed to convert path to string")
        .to_string()
}
