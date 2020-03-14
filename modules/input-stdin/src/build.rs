fn main() {
    // Add our app's build directory to the lib search path.
    println!("cargo:rustc-link-search=../../main/target/debug");
    // Add the app's dependency directory to the lib search path.
    // This is may be required if the app depends on any external "derive"
    // crates like the `dlopen_derive` crate that we add later.
    println!("cargo:rustc-link-search=../../main/target/debug/deps");
    // Link to the `app` crate library. This tells cargo to actually link
    // to the `app` crate that we include using `extern crate app;`.
    println!("cargo:rustc-link-lib=app");
}
