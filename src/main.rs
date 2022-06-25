use shadow_rs::shadow;

shadow!(build);
fn main() {
    println!("debug:{}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
    println!("branch:{}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'

    //shadow-rs built in const
    println!("version: {}", build::VERSION); // the version (description binary detail information)
    println!("clap long version: {}", build::CLAP_LONG_VERSION); // usually used by clap crates version() (description binary detail information)
    println!("{}", build::PKG_VERSION); // current package version. e.g. '1.3.15-beta2'
    println!("{}", build::PKG_VERSION_PRE); //current package minor version. e.g. 'beta2'
    println!("{}", build::BRANCH); // the branch, e.g. 'master'
    println!("{}", build::SHORT_COMMIT); // short commit hash, e.g. '8405e28e'
    println!("{}", build::RUST_CHANNEL); // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
    println!("{}", build::PROJECT_NAME); // your project name, e.g. 'shadow-rs'
    println!("{}", build::BUILD_RUST_CHANNEL); // e.g. 'debug'
}
