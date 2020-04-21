extern crate dbus;

use std::sync::Arc;
use std::time::Duration;
use dbus::blocking::LocalConnection;
use dbus::tree::{Factory, MTFn, MethodErr};
use dbus::tree::Tree;
use dbus::message::Message;
use dbus::tree::{MethodInfo, MethodResult, Signal};

use crate::devicereader::list_devices;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

pub struct Daemon {
    name: String,
    connection: LocalConnection,
    interval: u64,
    tree: Tree<MTFn, ()>

}
impl Daemon {
    pub fn new(name: &str) -> Daemon {
        let mut connection = LocalConnection::new_session().unwrap();
        connection.request_name(name, false, true, false);
        let f: Factory<MTFn, ()> = Factory::new_fn::<()>();

        let signal = Arc::new(f.signal("DevicesChecked", ()).sarg::<&str,_>("sender"));
        let signal2 = signal.clone();

        let tree = f.tree(()).add(f.object_path("/hello", ()).introspectable().add(
            f.interface("org.freedesktop.gpumanager", ()).add_m(

                f.method("ListDevices", (), move |m| {

                    let name: &str = m.msg.read1()?;
                    let mret = m.msg.method_return().append1(list_devices());

                    let sig = signal.msg(m.path.get_name(), m.iface.get_name())
                        .append1(&*name);

                    Ok(vec!(mret, sig))

                }).outarg::<&str,_>("reply")
                    .inarg::<&str,_>("name")

            ).add_s(signal2)

        )).add(f.object_path("/", ()).introspectable());
        Daemon {
            name: name.to_string(),
            connection: connection,
            interval: 1000,
            tree: tree
        }
    }
    pub fn start(mut self) -> () {
        self.tree.start_receive(&self.connection);
        loop {
            self.connection.process(Duration::from_millis(self.interval)).unwrap();
        }
    }
}