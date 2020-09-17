use std::{io, num::ParseFloatError, num::ParseIntError, str::FromStr, thread, time};

use io::Write;
use serialport::SerialPort;

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

#[derive(Debug, Default, Copy, Clone)]
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
    side_brush_ma: i32,
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

    fn set_motors(
        &mut self,
        left_distance: i32,
        right_distance: i32,
        speed: i32,
    ) -> std::io::Result<()>;
    fn get_motors(&mut self) -> Result<MotorStatus, GetDataError>;

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus, GetDataError>;
    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus, GetDataError>;
    fn get_charger(&mut self) -> Result<ChargerStatus, GetDataError>;

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
    ParseData(ParseIntError),
}

#[derive(Debug)]
pub enum ParseNumberError {
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
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

impl From<ParseIntError> for GetDataError {
    fn from(err: ParseIntError) -> GetDataError {
        GetDataError::ParseData(err)
    }
}

#[derive(Debug, PartialEq, Default)]
struct FloatField {
    name: String,
    value: f32,
}
#[derive(Debug, PartialEq, Default)]
struct IntField {
    name: String,
    value: i32,
}

impl FromStr for FloatField {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();
        let name = String::from(fields[0]);
        let value = fields[1].trim().parse::<f32>()?;

        Ok(FloatField {
            name: name,
            value: value,
        })
    }
}

impl FromStr for IntField {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();

        log::debug!("{:?}", fields);

        let name = String::from(fields[0]);
        let value = fields[1].trim().parse::<i32>()?;

        Ok(IntField {
            name: name,
            value: value,
        })
    }
}

impl NeatoRobot for DSeries<'_> {
    fn exit(&mut self) -> std::io::Result<()> {
        self.set_ldsrotation(Toggle::Off)?;
        self.set_testmode(Toggle::Off)?;
        // self.serial_port.flush()?;
        Ok(())
    }

    fn set_testmode(&mut self, value: Toggle) -> std::io::Result<()> {
        log::debug!("Setting testmode");
        writeln!(self.serial_port, "testmode {}", value.to_string())?;

        loop {
            let s = match self.read_line() {
                Ok(v) => {
                    println!("{}", v);
                    v
                }
                Err(_) => {
                    println!("Error reading back");
                    String::new()
                }
            };
            if s.contains("testmode") {
                println!("Serial port synced.");
                break;
            } else {
                println!("Serial port not yet in sync");
            }
        }

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
        thread::sleep(time::Duration::from_millis(5000));
        log::info!("Wait for laser turret to spin up to speed");
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
            let _n = self.serial_port.read(&mut buffer)?;
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

        let mut ranges = vec![];

        for _n in 1..363 {
            // 1 header line + 360? lines of distances for each degree + 1 trailing line
            let s = self.read_line()?;
            println!("{}", s);
            if !s.starts_with("ROTATION") && !s.starts_with("Angle") {
                let split = s.split(",");
                let vec: Vec<&str> = split.collect();
                let range_str = vec.get(1);
                match range_str {
                    Some(range_data) => {
                        match range_data.parse::<i32>() {
                            Ok(range) => ranges.push((range as f32) / 1000.0), // millimeters to meters
                            Err(err) => {
                                println!("Could not parse {}", s);
                                return Err(GetDataError::ParseData(err));
                            }
                        };
                    }
                    None => println!("Could not get element from {}", s),
                };
            };
        }
        log::debug!("Got scan_ranges");
        Ok(ranges)
    }

    fn set_motors(
        &mut self,
        left_distance: i32,
        right_distance: i32,
        speed: i32,
    ) -> std::io::Result<()> {
        log::debug!(
            "set_motors({}, {}, {})",
            left_distance,
            right_distance,
            speed
        );

        writeln!(
            self.serial_port,
            "setmotor {} {} {}\n",
            left_distance, right_distance, speed
        )?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;
        log::debug!("Set motors");
        Ok(())
    }

    fn get_motors(&mut self) -> Result<MotorStatus, GetDataError> {
        log::debug!("get_motors");

        writeln!(self.serial_port, "getmotors\n")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;

        log::debug!("Serial port flushed");

        let mut status = MotorStatus {
            ..Default::default()
        };

        log::debug!("Reading values...");
        loop {
            let header = self.read_line()?;
            log::debug!("{}", header);

            if header.contains("Parameter,Value") {
                break;
            }
        }

        for _n in 1..13 {
            // 13 fields
            let s = self.read_line()?;
            log::debug!("{}", s);
            let field = IntField::from_str(s.as_str())?;
            log::debug!("{:?}", field);
            match field.name.as_str() {
                "Brush_RPM" => status.brush_rpm = field.value,
                "Brush_mA" => status.brush_ma = field.value,
                "Vacuum_RPM" => status.vacuum_rpm = field.value,
                "Vacuum_mA" => status.vacuum_ma = field.value,
                "LeftWheel_RPM" => status.left_wheel_rpm = field.value,
                "LeftWheel_Load%" => status.left_wheel_load = field.value,
                "LeftWheel_PositionInMM" => status.left_wheel_position_in_mm = field.value,
                "LeftWheel_Speed" => status.left_wheel_speed = field.value,
                "RightWheel_RPM" => status.right_wheel_rpm = field.value,
                "RightWheel_Load%" => status.right_wheel_load = field.value,
                "RightWheel_PositionInMM" => status.right_wheel_position_in_mm = field.value,
                "RightWheel_Speed" => status.right_wheel_speed = field.value,
                "SideBrush_mA" => status.side_brush_ma = field.value,
                _ => log::error!("Unrecognized field: {:?}", field),
            }
        }
        log::debug!("Got motors");
        Ok(status)
    }

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus, GetDataError> {
        todo!()
    }

    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus, GetDataError> {
        todo!()
    }

    fn get_charger(&mut self) -> Result<ChargerStatus, GetDataError> {
        todo!()
    }

    fn set_backlight(&mut self, value: Toggle) -> std::io::Result<()> {
        writeln!(self.serial_port, "setled backlight{}", value.to_string())?;
        Ok(())
    }
}
