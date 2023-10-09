pub struct Device {
    pub device_id: String,
    pub model: String,
}

pub enum OfficeDevices {
    TableLED(Device),
    StandingRightLED(Device),
    StandingLeftLED(Device),
    WindowLED(Device),
    BoardLED(Device),
}
