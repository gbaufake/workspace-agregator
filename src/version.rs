use std::env;

// Get the version from Cargo.toml via env variable
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Get basic build info
pub fn get_build_info() -> String {
    format!(
        "workspace-aggregator v{}\nAuthor: {}\nDescription: {}",
        get_version(),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}

// Optional: Get detailed build info if using vergen
#[cfg(feature = "build-info")]
pub fn get_detailed_build_info() -> String {
    format!(
        "workspace-aggregator v{}\n\
         Build Time: {}\n\
         Profile: {}\n\
         Author: {}\n\
         Description: {}",
        get_version(),
        option_env!("VERGEN_BUILD_TIMESTAMP").unwrap_or("unknown"),
        option_env!("VERGEN_CARGO_PROFILE").unwrap_or("unknown"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION")
    )
}
