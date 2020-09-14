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
    let comms = serialport::open_with_settings(port, &s).unwrap();
    println!("Opened serial port");

    println!("Creating robot");
    let mut robot = DSeries::new(comms);
    println!("Create robot");

    robot.set_testmode(Toggle::On).unwrap();
    robot.set_ldsrotation(Toggle::On).unwrap();
    robot.request_scan().unwrap();
    match robot.get_scan_ranges()
    {
        Ok(ranges) => println!("{}", ranges.len()),
        Err(err) => {
            eprintln!("Could not get_scan_ranges: {:?}", err);
            robot.exit().unwrap();
        }
    }
    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);

    robot.exit().unwrap();
}
