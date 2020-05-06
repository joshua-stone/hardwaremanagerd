use std::fs::read_to_string;
use std::collections::HashMap;
use glob::glob;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Result, Write};

pub struct Cpu {
    pub cores: Vec<PathBuf>
}

pub fn write_to_core(core: &PathBuf, value: &str) -> Result<()> {
    let mut outfile = File::create(core)?;
    outfile.write_all(value.as_bytes())

}

pub fn detect_core_count() -> Vec<PathBuf>  {
    glob("/sys/devices/system/cpu/cpu[0-9]*").unwrap().map(|r| r.unwrap()).collect()
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            cores: detect_core_count()
        }
    }
}

pub fn disable_core(cpu: Cpu, core: i32) -> bool {
    let file = cpu.cores.get(core as usize).expect("Not a valid core");
    write_to_core(file, "0").is_ok()
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