use std::env;

pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn get_build_info() -> String {
    format!(
        "workspace-aggregator v{}\nAuthor: {}\nDescription: {}",
        get_version(),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}
