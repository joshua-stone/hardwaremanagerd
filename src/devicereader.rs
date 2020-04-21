extern crate udev;

use std::ffi::OsString;
use std::collections::HashMap;
use std::vec::IntoIter;
use std::collections::hash_map::RandomState;


pub fn list_devices() -> Vec<HashMap<String, String, RandomState>> {
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("pci").unwrap();
    enumerator.match_property("ID_PCI_CLASS_FROM_DATABASE", "Display controller").unwrap();

    let mut devices: Vec<HashMap<String, String>> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    for device in enumerator.scan_devices().unwrap() {
        properties.clear();
        for property in device.properties() {
            properties.insert(
                property.name().to_os_string().into_string().unwrap(),
                property.value().to_os_string().into_string().unwrap()
            );
        }
        devices.push(properties.clone());
    }
    devices
}
