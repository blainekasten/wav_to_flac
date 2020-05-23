#[cxx::bridge(namespace = flac)]
mod ffi {

            extern "C" {
                // One or more headers with the matching C++ declarations. Our code
                // generators don't read it but it gets #include'd and used in static
                // assertions to ensure our picture of the FFI boundary is accurate.
                // include!("lib-flac/include/share/compat.h");
                include!("lib-flac/src/flac/encode.h");

                fn encode_file() -> bool;
            }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use hound;

        let reader = hound::WavReader::open("testsamples/pop.wav").unwrap();

        // println!("{}", ffi::is_valid());
    }
}
