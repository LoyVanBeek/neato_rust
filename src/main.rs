use std::{env, thread, time};

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
        timeout: time::Duration::from_secs(1),

        ..Default::default()
    };

    println!("Opening serial port");
    let comms = serialport::open_with_settings(port, &s).expect("Failed to open port");
    println!("Opened serial port");

    println!("Creating robot");
    let mut robot = DSeries::new(comms);
    println!("Create robot");

    robot
        .set_testmode(Toggle::On)
        .expect("Failed to enable testmode");
    thread::sleep(time::Duration::from_millis(500));

    robot.set_backlight(Toggle::Off).unwrap();

    robot
        .set_ldsrotation(Toggle::On)
        .expect("Failed to enable LDS rotation");

    robot.request_scan().expect("Failed to request a scan");
    // thread::sleep(time::Duration::from_millis(500));

    match robot.get_scan_ranges() {
        Ok(ranges) => println!("{:?}", ranges),
        Err(err) => {
            eprintln!("Could not get_scan_ranges: {:?}", err);
            // robot.exit().expect("Failed to exit robot while handling err");
        }
    }
    // thread::sleep(time::Duration::from_secs(5));

    robot.set_motors(20, 20, 10).expect("Could not set motors");
    let motor_status = robot.get_motors().expect("Could not get motor data");
    println!("{:?}", motor_status);

    let analog_status = robot.get_analog_sensors().expect("Could not get analog sensors");
    println!("{:?}", analog_status);

    let digital_status = robot.get_digital_sensors().expect("Could not get digital sensors");
    println!("{:?}", digital_status);

    let charger_status = robot.get_charger().expect("Could not get charger status");
    println!("{:?}", charger_status);

    thread::sleep(time::Duration::from_secs(5));
    robot
        .set_motors(-20, -20, 10)
        .expect("Could not set motors");

    robot.set_backlight(Toggle::On).unwrap();

    robot.exit().expect("Failed to exit robot");
}
