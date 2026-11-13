fn main() {
    #[cfg(feature = "serialize")]
    println!("cargo::warning=Feature \"serialize\" was renamed to \"serde\". Change your `Cargo.toml` to avoid future breakage.");
}
