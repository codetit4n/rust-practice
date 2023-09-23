use sysinfo::{ComponentExt, System, SystemExt};

use std::{sync::mpsc::channel, thread, time::Duration};

enum Requests {
    LoadAvg,
    Temp,
}

fn main() {
    let (req_tx, req_rx) = channel();
    let (resp_tx, resp_rx) = channel();

    // worker thread
    thread::spawn(move || {
        let mut sys = System::new_all();

        for req in req_rx.iter() {
            sys.refresh_all();

            let msg = match req {
                Requests::LoadAvg => {
                    let load_avg = sys.load_average();
                    format!(
                        r#"Load average: {{"one-minute": {:.3}, "five-minutes": {:.3}, "fifteen-minutes": {:.3}}}"#,
                        load_avg.one, load_avg.five, load_avg.fifteen,
                    )
                }
                Requests::Temp => {
                    let mut soc_temps = Vec::new();
                    for component in sys.components() {
                        // SOC: System On a Chip - for Apple Silicon
                        if component.label().contains("SOC") {
                            let temp = component.temperature();
                            soc_temps.push(temp)
                        }
                    }
                    format!("Temperatures: {soc_temps:?}")
                }
            };

            resp_tx.send(msg).expect("Failed to send response");
        }
    });

    // printer thread
    thread::spawn(move || {
        for msg in resp_rx.iter() {
            println!("{msg}");
        }
    });

    loop {
        req_tx
            .send(Requests::LoadAvg)
            .expect("Failed to send load avg request");
        req_tx
            .send(Requests::Temp)
            .expect("Failed to send temp request");

        thread::sleep(Duration::from_secs(1));
    }
}
