use crate::bus::{DeviceRead, DeviceWrite};

pub struct Ppu {}

impl Ppu {
    pub fn new() -> Self {
        Self {}
    }
}

impl DeviceRead for Ppu {
    fn device_read(&self, address: u16) -> u8 {
        0
    }
}

impl DeviceWrite for Ppu {
    fn device_write(&mut self, address: u16, value: u8) {

    }
}
