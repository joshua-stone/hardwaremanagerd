extern crate clap;
use clap::App;
use dbus::blocking::Connection;
use std::time::Duration;
use std::collections::HashMap;
use std::ops::Div;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new("hwctl")
        .version("0.1")
        .about("Hardware manager utlity")
        .author("Joshua Stone")
        .get_matches();

    let conn = Connection::new_session()?;

    let mut proxy = conn.with_proxy("org.freedesktop.HardwareManager", "/gpu", Duration::from_millis(5000));

    let (gpus,): (Vec<HashMap<String, String>>,) = proxy.method_call("org.freedesktop.HardwareManager", "ListDevices", ("",))?;
    proxy = conn.with_proxy("org.freedesktop.HardwareManager", "/cpu", Duration::from_millis(5000));
    let (cpus,): (Vec<HashMap<String, String>>,) = proxy.method_call("org.freedesktop.HardwareManager", "ListFrequencies", ("",))?;

    proxy = conn.with_proxy("org.freedesktop.HardwareManager", "/ram", Duration::from_millis(5000));
    let (ram,): (Vec<HashMap<String, String>>,) = proxy.method_call("org.freedesktop.HardwareManager", "ListModules", ("",))?;

    for (number, values) in gpus.iter().enumerate() {
        println!("GPU #{}", number);
        println!("\tModel: {} {}", &values["ID_VENDOR_FROM_DATABASE"], &values["ID_MODEL_FROM_DATABASE"]);
        println!("\tDriver: {}", &values["DRIVER"]);
        println!("\tPCI ID: {}", &values["PCI_ID"]);
    }

    for (number, values) in cpus.iter().enumerate() {
        println!("CPU #{}", number);
        println!("\tFrequency: {:.2} GHz", values["scaling_cur_freq"].parse::<f32>().unwrap() * 0.000001);
        println!("\tGovernor:  {}",        values["scaling_governor"]);
    }

    for (number, values) in ram.iter().enumerate() {
        println!("RAM #{}", number);

        if values["speed"].eq("0") {
            println!("\t<EMPTY>");
            continue
        } else {
            println!("\tModel:   {} {}",   &values["manufacturer"], &values["model"]);
            println!("\tSize:    {} MB",   &values["size"]);
            println!("\tSpeed:   {} MT/s", &values["speed"]);
            println!("\tVoltage: {} V", &values["configured_voltage"].parse::<f32>().unwrap() * 0.001);
        }
    }
    Ok(())
}
