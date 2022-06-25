use shadow_rs::shadow;

shadow!(build);
pub fn default() -> String {
    format!(
        "{} {}+{}+{}+{}",
        build::PKG_VERSION,
        build::BRANCH,
        build::SHORT_COMMIT,
        build::BUILD_TARGET,
        build::BUILD_RUST_CHANNEL,
    )
}

pub fn with_name() -> String {
    format!(
        "{} {} {}+{}+{}+{}",
        build::PROJECT_NAME,
        build::PKG_VERSION,
        build::BRANCH,
        build::SHORT_COMMIT,
        build::BUILD_TARGET,
        build::BUILD_RUST_CHANNEL,
    )
}
