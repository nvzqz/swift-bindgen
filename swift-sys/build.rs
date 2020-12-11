fn main() {
    #[cfg(feature = "link")]
    println!("cargo:rustc-link-search=/usr/lib/swift/");
}
