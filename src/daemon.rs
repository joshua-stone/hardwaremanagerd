extern crate dbus;

use crate::devicereader::{get_memory_info, list_devices};

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
        let connection = LocalConnection::new_session().unwrap();
        connection.request_name(name, false, true, false).unwrap();
        let f: Factory<MTFn, ()> = Factory::new_fn::<()>();

        let signal = Arc::new(f.signal("DevicesChecked", ()).sarg::<&str, _>("sender"));
        let signal2 = signal.clone();
        let signal3 = Arc::new(f.signal("RamChecked", ()).sarg::<&str, _>("sender"));
        let signal4 = signal.clone();
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
                                let mret = m.msg.method_return().append1(get_memory_info());

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
