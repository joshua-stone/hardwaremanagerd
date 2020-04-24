extern crate libc;
extern crate udev;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::process::Command;

// This is a very hacky implementation; a proper solution would be parsing binary data found in
// /sys/firmware/dmi/tables/DMI, and polkit policies should be used instead of sudo. This is only a
// temporary implementation (*knocks on wood*) meant to test dbus capabilities for now
pub fn get_memory_info() -> Vec<HashMap<String, String, RandomState>> {
    let hwinfo = Command::new("sudo")
        .args(&["dmidecode", "--type", "17", "--quiet"])
        .output()
        .expect("");
    let output = String::from_utf8(hwinfo.stdout).unwrap();
    let lines: Vec<String> = output
        .split("\n")
        .filter(|i| !i.is_empty())
        .map(|i| String::from(i))
        .skip(1)
        .collect();

    let mut mem_info: Vec<HashMap<String, String>> = Vec::new();
    let mut mem_device: HashMap<String, String> = HashMap::new();
    for line in lines.iter() {
        if line.starts_with('\t') {
            let temp: Vec<String> = line.split(':').map(|i| String::from(i)).collect();
            let key: String = temp.first().unwrap().chars().skip(1).collect();
            let value: String = temp.last().unwrap().chars().skip(1).collect();
            mem_device.insert(key, value);
        } else {
            if !mem_device.is_empty() {
                mem_info.push(mem_device.clone());
                mem_device.clear();
            }
        }
    }
    mem_info.push(mem_device);
    mem_info
}
pub fn list_devices() -> Vec<HashMap<String, String, RandomState>> {
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("pci").unwrap();
    enumerator
        .match_property("ID_PCI_CLASS_FROM_DATABASE", "Display controller")
        .unwrap();

    let mut devices: Vec<HashMap<String, String>> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    for device in enumerator.scan_devices().unwrap() {
        properties.clear();
        for property in device.properties() {
            properties.insert(
                property.name().to_os_string().into_string().unwrap(),
                property.value().to_os_string().into_string().unwrap(),
            );
        }
        devices.push(properties.clone());
    }
    devices
}
