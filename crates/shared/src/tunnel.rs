use boringtun::device::{Device, DeviceConfig, DeviceHandle};
use boringtun::noise::Verbosity;

pub trait Tunnel {
    fn new(interface_name: String) -> Self;
    fn establish() -> anyhow::Result<()>;
}

pub struct WgTunnel {
    device_handle: DeviceHandle,
    interface_name: String
}

impl Tunnel for WgTunnel {
    fn new(interface_name: String) -> Self {
        let device_cfg = DeviceConfig {
            n_threads: 1,
            log_level: Verbosity::None,
        };

        return Self {
            interface_name,
            device_handle: DeviceHandle::new(interface_name.as_str())
        }
    }

    fn establish() -> anyhow::Result<()> {
        todo!()
    }
}
