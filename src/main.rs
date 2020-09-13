use neato_driver::{DSeries, NeatoRobot, Toggle};
use serialport::SerialPortSettings;

fn main() {
    println!("Hello robot!");

    let s = SerialPortSettings {
        baud_rate: 115200,
        ..Default::default()
    };
    
    let comms = serialport::open_with_settings("/dev/ttyACM1", &s).unwrap();
    let mut robot = DSeries::new(comms);

    robot.set_testmode(Toggle::On).unwrap();
    robot.request_scan().unwrap();
    robot.get_scan_ranges().unwrap();
}
