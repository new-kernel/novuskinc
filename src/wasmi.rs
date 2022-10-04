extern "C" {
    /// Runs a Wasm file, ``contents`` are the entire file's contents
    pub fn run_wasmi(contents: &[u8]);
}
