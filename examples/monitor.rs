use std::thread;

use macos_space_monitor::{MonitorEvent, SpaceMonitor};

fn main() {
    let (monitor, rx) = SpaceMonitor::new();
    let _monitoring_thread = thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            match event {
                MonitorEvent::SpaceChange(space) => {
                    println!("Space change detected! Active space is: {}", space);
                }
            }
        }
    });

    monitor.start_listening();
}
