extern crate dbus;

mod devicereader;
use devicereader::list_devices;

mod daemon;
use daemon::Daemon;

use std::sync::Arc;
use dbus::blocking::LocalConnection;
use dbus::tree::Factory;
use std::error::Error;
use std::time::Duration;
use dbus::Path;

fn main() -> () {
    let daemon = Daemon::new("org.freedesktop.gpumanager");
    daemon.start()
}
