fn main() -> shadow_rs::SdResult<()> {
    println!("cargo:rerun-if-changed=.git/logs/HEAD");
    shadow_rs::new()
}
