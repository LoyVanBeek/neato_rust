use neato_driver::DSeries;
use serialport::SerialPortSettings;

fn main() {
    println!("Hello robot!");

    let s = SerialPortSettings {
        baud_rate: 115200,
        ..Default::default()
    };
    
    let comms = serialport::open_with_settings("/dev/ttyACM1", &s).unwrap();
    let robot = DSeries::new(comms);
}
