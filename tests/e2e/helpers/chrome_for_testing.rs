pub fn find_chrome_for_testing_binary(subdir: &str, binary: &str) -> String {
    let base = std::env::var("CHROME_FOR_TESTING_DIR").unwrap_or_else(|_| {
        format!(
            "{}/chrome-for-testing",
            std::env::var("MISE_DATA_DIR").unwrap_or_else(|_| {
                format!(
                    "{}/.local/share/mise",
                    std::env::var("HOME").unwrap_or_default()
                )
            })
        )
    });

    let base_path = std::path::Path::new(&base).join(subdir);
    if let Ok(entries) = std::fs::read_dir(&base_path) {
        for entry in entries.flatten() {
            let path = entry.path().join(binary);
            if path.exists() {
                return path.to_string_lossy().to_string();
            }
        }
    }

    panic!(
        "Chrome for Testing binary not found at {base_path:?}. Run `mise run install-e2e` to install it."
    );
}
