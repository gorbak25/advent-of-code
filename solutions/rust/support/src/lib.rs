pub use load_file::load_str;

#[macro_export]
macro_rules! test_data {
    () => {{
        $crate::load_str!(
            (env!("CARGO_MANIFEST_DIR")
            .to_owned()
            .replace("solutions/rust/", "inputs/") + ".in")
            .replace("_A.in", ".in")
            .replace("_B.in", ".in")
            .as_str()
        )
    }}
}

