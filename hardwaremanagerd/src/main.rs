#[path = "lib/daemon.rs"] mod daemon;

use daemon::Daemon;

fn main() -> () {
    let daemon = Daemon::new("org.freedesktop.HardwareManager");
    daemon.start()
}
