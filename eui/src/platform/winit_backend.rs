use crate::runtime::contracts::{PlatformBackend, WindowMetrics};

pub struct WinitBackend {
    clipboard: Option<arboard::Clipboard>,
}

impl WinitBackend {
    pub fn new() -> Self {
        Self {
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

impl PlatformBackend for WinitBackend {
    fn should_close(&self) -> bool {
        false
    }

    fn poll_events(&mut self, _blocking: bool, _timeout_seconds: f64) {
        // Handled by winit event loop externally
    }

    fn query_metrics(&self) -> WindowMetrics {
        WindowMetrics::default()
    }

    fn get_clipboard_text(&mut self) -> String {
        self.clipboard.as_mut()
            .and_then(|cb| cb.get_text().ok())
            .unwrap_or_default()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        if let Some(cb) = self.clipboard.as_mut() {
            let _ = cb.set_text(text.to_string());
        }
    }

    fn now_seconds(&self) -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }
}
