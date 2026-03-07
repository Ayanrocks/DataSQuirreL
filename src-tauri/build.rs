/// Build script that reads the version from the root VERSION file
/// and exposes it as the `APP_VERSION` environment variable at compile time.
/// Access in Rust via `env!("APP_VERSION")`.
fn main() {
    let version = std::fs::read_to_string("../VERSION")
        .expect("VERSION file missing at project root")
        .trim()
        .to_string();

    println!("cargo:rustc-env=APP_VERSION={}", version);
    println!("cargo:rerun-if-changed=../VERSION");

    tauri_build::build();
}
