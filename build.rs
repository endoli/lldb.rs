fn main() {
    // These next 2 lines should only be for OS X and other platforms need
    // to do something different.
    println!("cargo:rustc-flags=-l framework=LLDB");
    println!("cargo:rustc-flags=-L framework=/Applications/Xcode.app/Contents/SharedFrameworks");
}
