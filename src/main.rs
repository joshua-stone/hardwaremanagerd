extern crate dbus;

mod daemon;
mod devicereader;
use daemon::Daemon;
use dbus::blocking::LocalConnection;
use dbus::tree::Factory;
use std::sync::Arc;
use std::time::Duration;

fn main() -> () {
    let daemon = Daemon::new("org.freedesktop.gpumanager");
    daemon.start()
}
