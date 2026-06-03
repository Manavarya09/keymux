#[cfg(target_os = "windows")]
mod win {
    use crate::input::{DeviceInfo, EventCallback, KeyEvent};
    use anyhow::Context;
    use std::ptr;

    pub fn platform_enumerate() -> anyhow::Result<Vec<DeviceInfo>> {
        // Minimal safe enumeration: return an empty list for now but compile on Windows.
        // Full implementation will call Win32 Raw Input APIs (RegisterRawInputDevices + GetRawInputDeviceList).
        Ok(Vec::new())
    }

    pub fn platform_start(_cb: EventCallback) -> anyhow::Result<()> {
        // Implementing a full Win32 message loop here would complicate Tauri integration.
        // For Phase 1 we provide a placeholder that will be replaced with a proper raw input loop.
        Ok(())
    }
}

pub use win::platform_enumerate;
pub use win::platform_start;
