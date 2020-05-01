use std::fs::read_to_string;
use std::collections::HashMap;
use glob::glob;
use std::path::PathBuf;

pub fn detect_core_count() -> Vec<PathBuf>  {
    glob("/sys/devices/system/cpu/cpu[0-9]*").unwrap().map(|r| r.unwrap()).collect()
}

pub fn list_core_frequencies(cpu_cores: Vec<PathBuf>) -> Vec<HashMap<String, String>> {
    let mut cores: Vec<HashMap<String, String>> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();

    for core in cpu_cores {
        let cpufreq = core.join("cpufreq");
        for entry in cpufreq.read_dir().expect("directory failed to open") {
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
            cores.push(properties.clone());
            properties.clear();
        }
    }
    cores
}