use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let load_avg = sys.load_average();
    println!(
        "one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
        load_avg.one, load_avg.five, load_avg.fifteen,
    );

    todo!();
}
