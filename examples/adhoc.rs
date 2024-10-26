use macos_space_monitor::SpaceMonitor;

fn main() {
    let space = SpaceMonitor::get_current_space_number();
    println!("Current space: {}", space);
}
