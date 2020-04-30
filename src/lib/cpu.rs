use std::fs::read_to_string;
use std::collections::HashMap;
use std::path::Path;

pub fn list_cores() -> Vec<HashMap<String, String>> {
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("cpu").unwrap();

    let mut cores: Vec<HashMap<String, String>> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    for device in enumerator.scan_devices().unwrap() {
        for property in device.properties() {
            if property.name().to_os_string().into_string().unwrap() == "DEVPATH" {
                let cpu_path: String = "/sys".to_owned() + property.value().to_str().unwrap();
                let path = Path::new(&cpu_path);
                let cpu_core = path.file_name().unwrap().to_str().unwrap()[3..].to_string();
                let cpufreq = path.join("cpufreq");
                for entry in cpufreq.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        let fpath = entry.path();
                        if fpath.is_file() {
                            let result = read_to_string(&fpath);

                            if let Ok(mut result) = result {
                                if result.ends_with("\n") {
                                    result.pop();
                                }
                                if result.ends_with(" ") {
                                    result.pop();
                                }
                                properties.insert(
                                    fpath.file_name().unwrap().to_str().unwrap().to_string(),
                                    result,
                                );
                            }
                        }
                    }

                }
                properties.insert("core".to_owned(), cpu_core);
            }
        }
        cores.push(properties.clone());
        properties.clear();
    }
    cores
}