pub trait Device {
    // Route table & network interface stuff
}

pub struct Info {
    interface_name: String,
}

pub struct Configuration {

}

#[cfg(target_os = "linux")]
impl Device for Linux {
    // Implement for Linux
}

#[cfg(target_os = "darwin")]
impl Device for Darwin {
    // Implement for MacOS
}

#[cfg(target_os = "windows")]
impl Device for WindowsOS {
    // Implement for Windows
}
