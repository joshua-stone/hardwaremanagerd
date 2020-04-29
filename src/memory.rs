extern crate smbios;

use std::collections::HashMap;
use smbios::Structure;
use smbios::stream;

// First value
const MEM_TYPES: [&str; 29] = [
    "Other",
    "Unknown",
    "DRAM",
    "EDRAM",
    "VRAM",
    "SRAM",
    "RAM",
    "ROM",
    "FLASH",
    "EEPROM",
    "FEPROM",
    "EPROM",
    "CDRAM",
    "3DRAM",
    "SDRAM",
    "SGRAM",
    "RDRAM",
    "DDR",
    "DDR2",
    "DDR2 FB-DIMM",
    "Reserved",
    "Reserved",
    "Reserved",
    "DDR3",
    "FBD2",
    "DDR4",
    "LPDDR",
    "LPDDR2",
    "LPDDR3"
];

pub struct MemoryStrings {
    pub locator: String,
    pub bank_locator: String,
    pub manufacturer: String,
    pub serial: String,
    pub model: String
}

pub fn create_mem(data: &Structure) -> HashMap<String, String> {
    let strings = &data.strings;

    let mem = MemoryStrings {
        locator:      strings.get(0).unwrap().to_owned(),
        bank_locator: strings.get(1).unwrap().to_owned(),
        manufacturer: strings.get(2).unwrap().to_owned(),
        serial:       strings.get(3).unwrap().to_owned(),
        model:        strings.get(4).unwrap().to_owned(),
    };

    let mut output: HashMap<String, String> = HashMap::new();
    output.insert(String::from("locator"), mem.locator);
    output.insert(String::from("bank_locator"), mem.bank_locator);
    output.insert(String::from("manufacturer"), mem.manufacturer);
    output.insert(String::from("serial"), mem.serial);
    output.insert(String::from("model"), mem.model);
    output.insert(String::from("speed"), u16::from_le_bytes([data.formatted[17], data.formatted[18]]).to_string());
    output.insert(String::from("type"), MEM_TYPES[data.formatted[14] as usize - 1].to_string());
    output
}

pub fn get_mem_info() -> Vec<HashMap<String, String>> {
    let DMI  = stream()
        .unwrap();
    let _dmi: Vec<Structure> = DMI.1;
    let dmi: Vec<HashMap<String, String>> = _dmi
        .iter().filter(|i| i.header.header_type == 17)

        .map(create_mem)
        .collect();
    dmi
}