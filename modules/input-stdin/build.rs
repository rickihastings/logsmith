fn main() {
    println!("cargo:rustc-link-search=../../target/debug");
    println!("cargo:rustc-link-search=../../target/debug/deps");
    println!("cargo:rustc-link-lib=logsmith");
}
