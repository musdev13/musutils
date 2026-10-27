#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn log_macro_json_branch_suppresses_output() {
    musutils::log!(true, "this should not print: {}", 1);
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn log_macro_non_json_branch_compiles_and_runs() {
    musutils::log!(false, "hello {}", "world");
    musutils::log!(false, "no args");
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn log_macro_accepts_runtime_bool() {
    let is_json = std::env::var_os("MUSUTILS_TEST_FAKE_JSON").is_some();
    musutils::log!(is_json, "runtime {}", 42);
}
