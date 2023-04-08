fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .compile("rust-epic");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
