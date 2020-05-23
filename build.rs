// build.rs

fn main() {
    std::env::set_var("CXXFLAGS", "-std=c++11");

    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        .include("lib-flac/include")
        .file("lib-flac/src/flac/encode.c")
        .compile("wav_to_flac");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
