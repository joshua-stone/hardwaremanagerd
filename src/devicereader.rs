extern crate udev;

use std::ffi::OsString;

pub fn list_devices() -> Vec<(OsString, OsString)> {
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("pci").unwrap();
    enumerator.match_property("ID_PCI_CLASS_FROM_DATABASE", "Display controller").unwrap();

    let mut devices: Vec<Vec<(OsString, OsString)>> = Vec::new();
    let mut properties: Vec<(OsString, OsString)> = Vec::new();
    for device in enumerator.scan_devices().unwrap() {
        properties.clear();
        for property in device.properties() {
            properties.push((property.name().to_os_string(), property.value().to_os_string()));
        }
        devices.push(properties.clone());
    }
    properties
}
