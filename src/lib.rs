use std::{thread, io, time};

use serialport::SerialPort;
use io::{Write};

#[derive(Debug)]
pub enum Toggle {
    On,
    Off,
}

impl ToString for Toggle {
    fn to_string(&self) -> String {
        match self {
            Toggle::On => String::from("on"),
            Toggle::Off => String::from("off"),
        }
    }
    // add code here
}

#[derive(Debug, Default)]
pub struct MotorStatus {
    brush_rpm: i32,
    brush_ma: i32,
    vacuum_rpm: i32,
    vacuum_ma: i32,
    left_wheel_rpm: i32,
    left_wheel_load: i32,
    left_wheel_position_in_mm: i32,
    left_wheel_speed: i32,
    right_wheel_rpm: i32,
    right_wheel_load: i32,
    right_wheel_position_in_mm: i32,
    right_wheel_speed: i32,
    side_bruch_ma: i32,
}

#[derive(Debug, Default)]
pub struct AnalogSensorStatus {
    battery_voltage: f32,
    battery_current: f32,
    battery_temperature: f32,
    external_voltage: f32,
    accelerometer_x: f32,
    accelerometer_y: f32,
    accelerometer_z: f32,
    vacuum_current: f32,
    side_brush_current: f32,
    mag_sensor_left: f32,
    mag_sensor_right: f32,
    wall_sensor: f32,
    drop_sensor_left: f32,
    drop_sensor_right: f32,
}

#[derive(Debug, Default)]
pub struct DigitalSensorStatus {
    sensor_dc_jack_is_in: bool,
    sensor_dustbin_is_in: bool,
    sensor_left_wheel_extended: bool,
    sensor_right_wheel_extended: bool,
    left_sidebit: bool,
    left_frontbit: bool,
    left_ldsbit: bool,
    right_sidebit: bool,
    right_frontbit: bool,
    right_ldsbit: bool,
}

#[derive(Debug, Default)]
pub struct ChargerStatus {
    fuel_percent: i32,
    battery_over_tmp: i32,
    charging_active: i32,
    charging_anabled: i32,
    confident_on_fuel: i32,
    on_reserved_fuel: i32,
    empty_fuel: i32,
    battery_failure: i32,
    ext_pwr_present: i32,
    thermistor_present: i32,
    batt_temp_c_avg: i32,
    v_batt_v_v: i32,
    v_ext_v: i32,
    charger_mah: i32,
    discharge_mah: i32,
}

pub trait NeatoRobot {
    fn exit(&mut self) -> std::io::Result<()>;
    fn set_testmode(&mut self, value: Toggle) -> std::io::Result<()>;
    fn set_ldsrotation(&mut self, value: Toggle) -> std::io::Result<()>;

    fn request_scan(&mut self) -> std::io::Result<()>;
    fn get_scan_ranges(&mut self) -> Result<Vec<f32>, GetDataError>;

    fn set_motors(&mut self, left_distance: i32, right_distance: i32, speed: i32) -> std::io::Result<()>;
    fn get_motors(&mut self) -> Result<MotorStatus, std::io::Error>;

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus, std::io::Error>;
    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus, std::io::Error>;
    fn get_charger(&mut self) -> Result<ChargerStatus, std::io::Error>;

    fn set_backlight(&mut self, value: Toggle) -> std::io::Result<()>;

    fn read_line(&mut self) -> Result<String, GetDataError>;
}


pub struct DSeries<'a> {
    serial_port: Box<dyn SerialPort + 'a>,
    // motor_status: MotorStatus,
    // analog_sensor_status: AnalogSensorStatus,
    // digital_sensor_status: DigitalSensorStatus,
    // charger_status: ChargerStatus,
}

impl DSeries<'_> {
    pub fn new(serial_port: Box<dyn SerialPort>) -> Self {
        Self {
            serial_port: serial_port,
            // motor_status: MotorStatus{..Default::default()},
            // analog_sensor_status: AnalogSensorStatus{..Default::default()},
            // digital_sensor_status: DigitalSensorStatus{..Default::default()},
            // charger_status: ChargerStatus{..Default::default()},
        }
    }
}

#[derive(Debug)]
pub enum GetDataError {
    Io(io::Error),
    Parse(std::string::FromUtf8Error),
}
impl From<io::Error> for GetDataError {
    fn from(err: io::Error) -> GetDataError {
        GetDataError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for GetDataError {
    fn from(err: std::string::FromUtf8Error) -> GetDataError {
        GetDataError::Parse(err)
    }
}

impl NeatoRobot for DSeries <'_> {
    fn exit(&mut self) -> std::io::Result<()>{
        self.set_ldsrotation(Toggle::Off)?;
        self.set_testmode(Toggle::Off)?;
        // self.serial_port.flush()?;
        Ok(())
    }

    fn set_testmode(&mut self, value: Toggle) -> std::io::Result<()>{
        log::debug!("Setting testmode");
        writeln!(self.serial_port, "testmode {}", value.to_string())?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;
        log::debug!("Set testmode");
        Ok(())
    }

    fn set_ldsrotation(&mut self, value: Toggle) -> std::io::Result<()> {
        log::debug!("Setting ldsrotation");
        writeln!(self.serial_port, "setldsrotation {}", value.to_string())?;
                
        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;
        log::debug!("Set ldsrotation");
        Ok(())
    }

    fn request_scan(&mut self) -> std::io::Result<()> {
        log::debug!("Requesting scan");
        writeln!(self.serial_port, "getldsscan")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;
        log::debug!("Port flushed");

        log::debug!("Requested scan");
        Ok(())
    }

    fn read_line(&mut self) -> Result<String, GetDataError> {
        let mut longbuffer = vec![];

        for _n in 1..100 {
            let mut buffer = [0; 1];
            let n = self.serial_port.read(&mut buffer)?;
            // println!("buffer: {}, {:?}", n, &buffer[..n]);
            let ch = buffer[0];
            if ch as char == '\n' {
                break;
            }
            longbuffer.push(ch);
        }

        let s = String::from_utf8(longbuffer)?;
        return Ok(s);
    }

    fn get_scan_ranges(&mut self) -> Result<Vec<f32>, GetDataError> {
        log::debug!("Reading serial_port for scan_ranges");
        for _n in 1..363 {  // 1 header line + 360? lines of distances for each degree + 1 trailing line
            let s = self.read_line()?;
            println!("{}", s);
        }
        log::debug!("Got scan_ranges");
        Ok(vec![])
    }

    fn set_motors(&mut self, _left_distance: i32, _right_distance: i32, _speed: i32) -> std::io::Result<()> {
        todo!()
    }

    fn get_motors(&mut self) -> Result<MotorStatus, std::io::Error> {
        todo!()
    }

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus, std::io::Error> {
        todo!()
    }

    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus, std::io::Error> {
        todo!()
    }

    fn get_charger(&mut self) -> Result<ChargerStatus, std::io::Error> {
        todo!()
    }

    fn set_backlight(&mut self, value: Toggle) -> std::io::Result<()> {
        writeln!(self.serial_port, "setled backlight{}", value.to_string())?;
        Ok(())
    }
}
