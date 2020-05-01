extern crate dbus;
#[path = "memory.rs"] mod memory;
#[path = "devicereader.rs"] mod devicereader;
#[path = "cpu.rs"] mod cpu;

use memory::get_mem_info;
use devicereader::list_devices;
use cpu::list_core_frequencies;
use cpu::detect_core_count;

use dbus::blocking::LocalConnection;
use dbus::tree::Tree;
use dbus::tree::{Factory, MTFn};
use std::sync::Arc;
use std::time::Duration;

pub struct Daemon {
    name: String,
    connection: LocalConnection,
    interval: u64,
    tree: Tree<MTFn, ()>,
}

impl Daemon {
    pub fn new(name: &str) -> Daemon {
        let cpu_cores = detect_core_count();
        let connection = LocalConnection::new_session().unwrap();
        connection.request_name(name, false, true, false).unwrap();
        let f: Factory<MTFn, ()> = Factory::new_fn::<()>();

        // Going to need to learn how to structure dbus signals to avoid boilerplate like the lines
        // below:
        let signal = Arc::new(f.signal("DevicesChecked", ()).sarg::<&str, _>("sender"));
        let signal2 = signal.clone();
        let signal3 = Arc::new(f.signal("RamChecked", ()).sarg::<&str, _>("sender"));
        let signal4 = signal3.clone();
        let signal5 = Arc::new(f.signal("CPUChecked", ()).sarg::<&str, _>("sender"));
        let signal6 = signal5.clone();
        let tree = f
            .tree(())
            .add(
                f.object_path("/gpu", ()).introspectable().add(
                    f.interface("org.freedesktop.gpumanager", ())
                        .add_m(
                            f.method("ListDevices", (), move |m| {
                                let name: &str = m.msg.read1()?;
                                let mret = m.msg.method_return().append1(list_devices());

                                let sig = signal
                                    .msg(m.path.get_name(), m.iface.get_name())
                                    .append1(&*name);

                                Ok(vec![mret, sig])
                            })
                            .outarg::<&str, _>("reply")
                            .inarg::<&str, _>("name"),
                        )
                        .add_s(signal2),
                ),
            )
            .add(
                f.object_path("/ram", ()).introspectable().add(
                    f.interface("org.freedesktop.gpumanager", ())
                        .add_m(
                            f.method("ListModules", (), move |m| {
                                let name: &str = m.msg.read1()?;
                                let mret = m.msg.method_return().append1(get_mem_info());

                                let sig = signal3
                                    .msg(m.path.get_name(), m.iface.get_name())
                                    .append1(&*name);

                                Ok(vec![mret, sig])
                            })
                            .outarg::<&str, _>("reply")
                            .inarg::<&str, _>("name"),
                        )
                        .add_s(signal4),
                ),
            )
            .add(
                f.object_path("/cpu", ()).introspectable().add(
                    f.interface("org.freedesktop.gpumanager", ())
                        .add_m(
                            f.method("ListFrequencies", (), move |m| {
                                let name: &str = m.msg.read1()?;
                                let mret = m.msg.method_return().append1(list_core_frequencies(cpu_cores.clone()));

                                let sig = signal5
                                    .msg(m.path.get_name(), m.iface.get_name())
                                    .append1(&*name);

                                Ok(vec![mret, sig])
                            })
                                .outarg::<&str, _>("reply")
                                .inarg::<&str, _>("name"),
                        )
                        .add_s(signal6),
                ),
            )
            .add(f.object_path("/", ()).introspectable());
        Daemon {
            name: name.to_string(),
            connection: connection,
            interval: 1000,
            tree: tree,
        }
    }
    pub fn start(mut self) -> () {
        self.tree.start_receive(&self.connection);

        loop {
            self.connection
                .process(Duration::from_millis(self.interval))
                .unwrap();
        }
    }
}