extern crate udev;

use std::io;

fn main() -> io::Result<()>  {
    let mut enumerator = udev::Enumerator::new()?;

    enumerator.match_subsystem("pci")?;
    enumerator.match_property("ID_PCI_CLASS_FROM_DATABASE", "Display controller")?;

    for device in enumerator.scan_devices()? {
        println!();
        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }
        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }
    Ok(())
}
