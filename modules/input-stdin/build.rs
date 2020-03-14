fn main() {
    println!("cargo:rustc-link-search=../../logsmith/target/debug");
    println!("cargo:rustc-link-search=../../logsmith/target/debug/deps");
    println!("cargo:rustc-link-lib=logsmith");
}
