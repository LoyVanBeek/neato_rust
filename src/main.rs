use std::{time, thread, env};

use neato_driver::{DSeries, NeatoRobot, Toggle};
use serialport::SerialPortSettings;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let port = &args[1];

    env_logger::init();
    
    println!("Hello robot on port {}!", port);

    let s = SerialPortSettings {
        baud_rate: 115200,
        ..Default::default()
    };
    
    println!("Opening serial port");
    let comms = serialport::open_with_settings(port, &s).expect("Failed to open port");
    println!("Opened serial port");

    println!("Creating robot");
    let mut robot = DSeries::new(comms);
    println!("Create robot");

    robot.set_testmode(Toggle::On).expect("Failed to enable testmode");
    robot.set_ldsrotation(Toggle::On).expect("Failed to enable LDS rotation");
    robot.request_scan().expect("Failed to request a scan");
    match robot.get_scan_ranges()
    {
        Ok(ranges) => println!("{}", ranges.len()),
        Err(err) => {
            eprintln!("Could not get_scan_ranges: {:?}", err);
            robot.exit().expect("Failed to exit robot while handling err");
        }
    }
    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);

    robot.exit().expect("Failed to exit robot");
}
