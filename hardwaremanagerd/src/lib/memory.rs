extern crate smbios;

use std::collections::HashMap;
use smbios::Structure;
use smbios::stream;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

const FORM_FACTORS: [&str; 16] = [
    "Other",
    "Unknown",
    "SIMM",
    "SIP",
    "Chip",
    "DIP",
    "ZIP",
    "Proprietary Card",
    "DIMM",
    "TSOP",
    "Row of chips",
    "RIMM",
    "SODIMM",
    "SRIMM",
    "FB-DIMM",
    "Die"
];
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
    pub total_width:        u16,    // 08h
    pub data_width:         u16,    // 0Ah
    pub size:               u16,    // 0Ch
    pub form_factor:        String, // 0Eh
    pub device_set:         u8,     // 0Fh
    pub device_locator:     String, // 10h
    pub bank_locator:       String, // 11h
    pub memory_type:        String, // 12h
    pub type_detail:        u16,    // 13h
    pub speed:              u16,    // 15h
    pub manufacturer:       String, // 17h
    pub serial_number:      String, // 18h
    pub asset_tag:          String, // 19h
    pub part_number:        String, // 1Ah
    pub extended_size:      u32,    // 1Ch
    pub configured_speed:   u16,    // 20h
    pub minimum_voltage:    u16,    // 22h
    pub maximum_voltage:    u16,    // 24h
    pub configured_voltage: u16     // 26h
}

pub fn create_mem(data: &Structure) -> HashMap<String, String> {
    let strings = &data.strings;

    let mem = MemoryStrings {
        total_width:        u16::from_le_bytes([data.formatted[4], data.formatted[5]]),
        data_width:         u16::from_le_bytes([data.formatted[6], data.formatted[7]]),
        size:               u16::from_le_bytes([data.formatted[8], data.formatted[9]]),
        form_factor:        FORM_FACTORS[data.formatted[10] as usize - 1].to_string(),
        device_set:         data.formatted[11],
        device_locator:     strings.get(0).unwrap().to_owned(),
        bank_locator:       strings.get(1).unwrap().to_owned(),
        memory_type:        MEM_TYPES[data.formatted[14] as usize - 1].to_string(),
        type_detail:        u16::from_le_bytes([data.formatted[15], data.formatted[16]]),
        speed:              u16::from_le_bytes([data.formatted[17], data.formatted[18]]),
        manufacturer:       strings.get(2).unwrap().to_owned(),
        serial_number:      strings.get(3).unwrap().to_owned(),
        asset_tag:          strings.get(4).unwrap().to_owned(),
        part_number:        strings.get(5).unwrap().trim_end().to_string(),
        extended_size:      Cursor::new(data.formatted[24..=27].to_vec()).read_u32::<LittleEndian>().unwrap(),
        configured_speed:   u16::from_le_bytes([data.formatted[28], data.formatted[29]]),
        minimum_voltage:    u16::from_le_bytes([data.formatted[30], data.formatted[31]]),
        maximum_voltage:    u16::from_le_bytes([data.formatted[32], data.formatted[33]]),
        configured_voltage: u16::from_le_bytes([data.formatted[34], data.formatted[35]])
    };

    [(String::from("locator"),           mem.device_locator),
    (String::from("bank_locator"),       mem.bank_locator),
    (String::from("manufacturer"),       mem.manufacturer),
    (String::from("serial"),             mem.serial_number),
    (String::from("asset_tag"),          mem.asset_tag),
    (String::from("model"),              mem.part_number),
    (String::from("speed"),              mem.speed.to_string()),
    (String::from("total_width"),        mem.total_width.to_string()),
    (String::from("data_width"),         mem.total_width.to_string()),
    (String::from("size"),               mem.size.to_string()),
    (String::from("form_factor"),        mem.form_factor),
    (String::from("device_set"),         mem.device_set.to_string()),
    (String::from("memory_type"),        mem.memory_type),
    (String::from("extended"),           mem.extended_size.to_string()),
    (String::from("configured_speed"),   mem.configured_speed.to_string()),
    (String::from("minimum_voltage"),    mem.minimum_voltage.to_string()),
    (String::from("maximum_voltage"),    mem.maximum_voltage.to_string()),
    (String::from("configured_voltage"), mem.configured_voltage.to_string()),
    ].iter().cloned().collect()
}

pub fn get_mem_info() -> Vec<HashMap<String, String>> {
    let dmi: Vec<Structure> = stream().unwrap().1;

    dmi.iter()
        .filter(|i| i.header.header_type == 17)
        .map(create_mem)
        .collect()
}