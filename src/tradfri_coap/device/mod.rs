// https://github.com/home-assistant-libs/pytradfri/blob/master/pytradfri/const.py

use {super::device_worker::DeviceWorker, serde::Deserialize};

pub mod colournames;
pub mod light;
pub mod outlet;

#[derive(Debug, Deserialize)]
pub(crate) struct DeviceInfo {
    #[serde(rename = "0")]
    pub mfr: String,
    #[serde(rename = "1")]
    pub device_name: String,
    #[serde(rename = "3")]
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct BasicDevice {
    #[serde(rename = "5750")]
    pub device_type: u32,
}

#[derive(Debug, Clone)]
pub enum Device {
    RemoteControl,       // 0
    Light(light::Light), // 2
    // Panel,
    // Door,
    // RecessedSpotlight,
    // Driver,
    ControlOutlet(outlet::Outlet),
    // SquareCeilingWallLamp,
    // RoundCeilingWallLamp,
    // SignalRepeater,
    // Blind,
    // WirelessDimmer,
    // MotionSensor,
    // OnOffSwitchDimmer,
    // OpenCloseRemote
}

impl Device {
    pub fn new(worker: DeviceWorker, bytes: &[u8]) -> super::Result<Self> {
        let basic_device: BasicDevice = serde_json::from_slice(bytes)?;

        match basic_device.device_type {
            0 => Ok(Self::RemoteControl),
            // 1 => Ok(Self::...),
            2 => Ok(Self::Light(light::Light::new(worker, bytes)?)),
            3 => Ok(Self::ControlOutlet(outlet::Outlet::new(worker, bytes)?)),
            // 4 => Ok(Self::...),
            // 5 => Ok(Self::...),
            // ...
            n => Err(super::Error::new(format!("Unsupported device no. {}", n))),
        }
    }
}
