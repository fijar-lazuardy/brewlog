use std::time::Duration;

use thirtyfour::prelude::*;

pub struct BrowserSession {
    pub driver: WebDriver,
    pub base_url: String,
}

impl BrowserSession {
    pub async fn new(base_url: &str) -> WebDriverResult<Self> {
        let port = super::chromedriver::ensure_chromedriver();
        let chromedriver_url = format!("http://localhost:{port}");

        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless()?;
        caps.add_arg("--no-sandbox")?;
        caps.add_arg("--disable-gpu")?;
        caps.add_arg("--disable-dev-shm-usage")?;
        caps.add_arg("--window-size=1280,1024")?;
        caps.set_binary(&find_chrome_binary())?;

        let driver = WebDriver::new(&chromedriver_url, caps).await?;
        driver
            .set_implicit_wait_timeout(Duration::from_secs(2))
            .await?;

        Ok(Self {
            driver,
            base_url: base_url.to_string(),
        })
    }

    pub async fn goto(&self, path: &str) -> WebDriverResult<()> {
        self.driver
            .goto(&format!("{}{}", self.base_url, path))
            .await
    }

    pub async fn quit(self) {
        let _ = self.driver.quit().await;
    }
}

fn find_chrome_binary() -> String {
    find_chrome_for_testing_binary("chrome", "chrome-linux64/chrome")
}

fn find_chrome_for_testing_binary(subdir: &str, binary: &str) -> String {
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
