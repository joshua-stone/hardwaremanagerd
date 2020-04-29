#[path = "lib/daemon.rs"] mod daemon;

use daemon::Daemon;

/*use lib::memory;

use lib::daemon::Daemon;
use dbus::blocking::LocalConnection;
use dbus::tree::Factory;
use std::sync::Arc;
use std::time::Duration;

use memory::get_mem_info;
use daemon::Daemon;*/


fn main() -> () {
    let daemon = Daemon::new("org.freedesktop.gpumanager");
    daemon.start()
}