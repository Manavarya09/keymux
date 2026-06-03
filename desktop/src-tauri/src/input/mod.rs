#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod unix;

use serde::Serialize;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Serialize, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
}

pub type EventCallback = Box<dyn Fn(KeyEvent) + Send + 'static>;

#[derive(Debug, Serialize, Clone)]
pub struct KeyEvent {
    pub device: String,
    pub keycode: u32,
    pub pressed: bool,
}

pub fn enumerate_devices() -> anyhow::Result<Vec<DeviceInfo>> {
    platform_enumerate()
}

pub fn start_listening(cb: EventCallback) -> anyhow::Result<()> {
    platform_start(cb)
}

#[cfg(target_os = "windows")]
use windows::{platform_enumerate, platform_start};

#[cfg(not(target_os = "windows"))]
use unix::{platform_enumerate, platform_start};
