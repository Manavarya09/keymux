use crate::input::DeviceInfo;
use crate::input::EventCallback;

pub fn platform_enumerate() -> anyhow::Result<Vec<DeviceInfo>> {
    // Non-windows stub to allow cross-platform builds
    Ok(vec![])
}

pub fn platform_start(_cb: EventCallback) -> anyhow::Result<()> {
    // Not implemented on non-Windows yet
    Ok(())
}
