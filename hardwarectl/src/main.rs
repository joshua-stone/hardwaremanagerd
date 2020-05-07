extern crate clap;
use clap::App;
use dbus::blocking::Connection;
use std::time::Duration;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new("hwctl")
        .version("0.1")
        .about("Hardware manager utlity")
        .author("Joshua Stone")
        .get_matches();

    let conn = Connection::new_session()?;

    let proxy = conn.with_proxy("org.freedesktop.HardwareManager", "/gpu", Duration::from_millis(5000));

    let (gpus,): (Vec<HashMap<String, String>>,) = proxy.method_call("org.freedesktop.HardwareManager", "ListDevices", ("",))?;
    let proxy = conn.with_proxy("org.freedesktop.HardwareManager", "/cpu", Duration::from_millis(5000));
    let (cpus,): (Vec<HashMap<String, String>>,) = proxy.method_call("org.freedesktop.HardwareManager", "ListFrequencies", ("",))?;

    for name in gpus {
        let model = &name["ID_MODEL_FROM_DATABASE"];
        let vendor = &name["ID_VENDOR_FROM_DATABASE"];
        let driver = &name["DRIVER"];
        let pci_id = &name["PCI_ID"];

        println!("{}", model);
        println!("\tModel:  {}", vendor);
        println!("\tDriver: {}", driver);
        println!("\tPCI ID: {}", pci_id);
    }

    for (core, values) in cpus.iter().enumerate() {
        let current_frequency = &values["scaling_cur_freq"];
        let governor = &values["scaling_governor"];
        println!("CPU #{}", core);
        println!("\tFrequency: {}", current_frequency);
        println!("\tGovernor:  {}", governor);
    }
    Ok(())
}
