use std::path::PathBuf;
use std::process::Command;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn assert_preprocess_case(name: &str) {
    let script = PathBuf::from(MANIFEST_DIR).join("preprocess_run.mjs");

    let output = Command::new("node")
        .arg(&script)
        .arg(name)
        .output()
        .unwrap_or_else(|err| panic!("[{name}] failed to spawn node for preprocess case: {err}"));

    if !output.status.success() {
        panic!(
            "[{name}] preprocess parity check failed\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
