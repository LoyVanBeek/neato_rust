use anyhow::Context;
use std::{fmt::Display, io, num::ParseFloatError, num::ParseIntError, str::FromStr, thread, time};

use io::Write;
use serialport::SerialPort;

use anyhow::Result;

use thiserror::Error;


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

#[derive(Debug, Default, Clone, Copy)]
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

impl FromStr for MotorStatus {
    // add code here
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut status = MotorStatus {
            ..Default::default()
        };

        for line in lines {
            log::debug!("line: {}", line);
            if !line.is_empty(){
                let field = IntField::from_str(line)?;
                log::debug!("field: {:?}", field);
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
        }

        Ok(status)
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

impl FromStr for AnalogSensorStatus {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut status = AnalogSensorStatus {
            ..Default::default()
        }; 

        for line in lines {
            log::debug!("line: {}", line);
            if !line.is_empty(){
                let field = UnitFloatField::from_str(line)?;
                log::debug!("{:?}", field);
                match field.name.as_str() {
                    "BatteryVoltage" => status.battery_voltage = field.value,
                    "BatteryCurrent" => status.battery_current = field.value,
                    "BatteryTemperature" => status.battery_temperature = field.value,
                    "ExternalVoltage" => status.external_voltage = field.value,
                    "AccelerometerX" => status.accelerometer_x = field.value,
                    "AccelerometerY" => status.accelerometer_y = field.value,
                    "AccelerometerZ" => status.accelerometer_z = field.value,
                    "VacuumCurrent" => status.vacuum_current = field.value,
                    "SideBrushCurrent" => status.side_brush_current = field.value,
                    "MagSensorLeft" => status.mag_sensor_left = field.value,
                    "MagSensorRight" => status.mag_sensor_right = field.value,
                    "WallSensor" => status.wall_sensor = field.value,
                    "DropSensorLeft" => status.drop_sensor_left = field.value,
                    "DropSensorRight" => status.drop_sensor_right = field.value,
                    _ => log::error!("Unrecognized field: {:?}", field),
                }
            }
        }

        Ok(status)
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

impl FromStr for DigitalSensorStatus {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut status = DigitalSensorStatus {
            ..Default::default()
        }; 

        for line in lines {
            log::debug!("line: {}", line);
            if !line.is_empty(){
                let field = BoolField::from_str(line)?;
                log::debug!("{:?}", field);
                match field.name.as_str() {
                    "SNSR_DC_JACK_IS_IN" => status.sensor_dc_jack_is_in = field.value,
                    "SNSR_DUSTBIN_IS_IN" => status.sensor_dustbin_is_in = field.value,
                    "SNSR_LEFT_WHEEL_EXTENDED" => status.sensor_left_wheel_extended = field.value,
                    "SNSR_RIGHT_WHEEL_EXTENDED" => status.sensor_right_wheel_extended = field.value,
                    "LSIDEBIT" => status.left_sidebit = field.value,
                    "LFRONTBIT" => status.left_frontbit = field.value,
                    "LLDSBIT" => status.left_ldsbit = field.value,
                    "RSIDEBIT" => status.right_sidebit = field.value,
                    "RFRONTBIT" => status.right_frontbit = field.value,
                    "RLDSBIT" => status.right_ldsbit = field.value,
                    _ => log::error!("Unrecognized field: {:?}", field),
                }
            }
        }

        Ok(status)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ChargerStatus {
    fuel_percent: i32,
    battery_over_tmp: i32,
    charging_active: i32,
    charging_enabled: i32,
    confident_on_fuel: i32,
    on_reserved_fuel: i32,
    empty_fuel: i32,
    battery_failure: i32,
    ext_pwr_present: i32,
    thermistor_present: i32,
    batt_temp_c_avg: i32,
    v_batt_v_v: f32,
    v_ext_v: f32,
    charger_mah: i32,
    discharge_mah: i32,
}

pub trait NeatoRobot {
    fn exit(&mut self) -> Result<()>;
    fn set_testmode(&mut self, value: Toggle) -> Result<()>;
    fn set_ldsrotation(&mut self, value: Toggle) -> Result<()>;

    fn request_scan(&mut self) -> Result<()>;
    fn get_scan_ranges(&mut self) -> Result<Vec<f32>>;

    fn set_motors(
        &mut self,
        left_distance: i32,
        right_distance: i32,
        speed: i32,
    ) -> Result<()>;
    fn get_motors(&mut self) -> Result<MotorStatus>;

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus>;
    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus>;
    fn get_charger(&mut self) -> Result<ChargerStatus>;

    fn set_backlight(&mut self, value: Toggle) -> Result<()>;

    fn read_line(&mut self) -> Result<String>;
    fn read_lines(&mut self, line_count: i32) -> Result<String>;
}

pub struct DSeries<'a> {
    serial_port: Box<dyn SerialPort + 'a>,
    motor_status: MotorStatus,
    analog_sensor_status: AnalogSensorStatus,
    digital_sensor_status: DigitalSensorStatus,
    charger_status: ChargerStatus,
}

impl Display for DSeries<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.motor_status,
            self.analog_sensor_status,
            self.digital_sensor_status,
            self.charger_status
        )
    }
}

impl DSeries<'_> {
    pub fn new(serial_port: Box<dyn SerialPort>) -> Self {
        Self {
            serial_port: serial_port,
            motor_status: MotorStatus {
                ..Default::default()
            },
            analog_sensor_status: AnalogSensorStatus {
                ..Default::default()
            },
            digital_sensor_status: DigitalSensorStatus {
                ..Default::default()
            },
            charger_status: ChargerStatus {
                ..Default::default()
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum GetDataError {
    #[error("Error communicating with the robot")]
    Io(io::Error),
    #[error("Error parsing bytes to string")]
    Parse(std::string::FromUtf8Error),
    #[error("Error parsing string to int")]
    ParseIntData(ParseIntError),
    #[error("Error parsing string to float")]
    ParseFloatData(ParseFloatError),
}

#[derive(Error, Debug)]
pub enum ParseNumberError {
    #[error("Error parsing string to int")]
    ParseInt(ParseIntError),
    #[error("Error parsing string to float")]
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
        GetDataError::ParseIntData(err)
    }
}
impl From<ParseFloatError> for GetDataError {
    fn from(err: ParseFloatError) -> GetDataError {
        GetDataError::ParseFloatData(err)
    }
}

#[derive(Debug, PartialEq, Default)]
struct UnitFloatField {
    name: String,
    unit: String,
    value: f32,
}
#[derive(Debug, PartialEq, Default)]
struct IntField {
    name: String,
    value: i32,
}
#[derive(Debug, PartialEq, Default)]
struct BoolField {
    name: String,
    value: bool,
}
#[derive(Debug, PartialEq, Default)]
struct SimpleFloatField {
    name: String,
    value: f32,
}

impl FromStr for UnitFloatField {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();
        let name = String::from(fields[0]);
        let unit = String::from(fields[1]);
        let value = fields[2].trim().parse::<f32>()?;

        Ok(UnitFloatField {
            name: name,
            unit: unit,
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

impl FromStr for BoolField {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();

        log::debug!("{:?}", fields);

        let name = String::from(fields[0]);
        let value = fields[1].trim().parse::<i32>()?;

        Ok(BoolField {
            name: name,
            value: value == 1,
        })
    }
}

impl FromStr for SimpleFloatField {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();
        let name = String::from(fields[0]);
        let value = fields[1].trim().parse::<f32>()?;

        Ok(SimpleFloatField {
            name: name,
            value: value,
        })
    }
}

impl NeatoRobot for DSeries<'_> {
    fn exit(&mut self) -> Result<()> {
        self.set_ldsrotation(Toggle::Off)?;
        self.set_testmode(Toggle::Off)?;
        // self.serial_port.flush()?;
        Ok(())
    }

    fn set_testmode(&mut self, value: Toggle) -> Result<()> {
        log::debug!("Setting testmode");
        writeln!(self.serial_port, "testmode {}", value.to_string()).context("Could not write to serial port")?;

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

    fn set_ldsrotation(&mut self, value: Toggle) -> Result<()> {
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

    fn request_scan(&mut self) -> Result<()> {
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

    fn read_line(&mut self) -> Result<String> {
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

    fn read_lines(&mut self, line_count: i32) -> Result<String> {
        let mut lines = vec![];

        for _l in 1..line_count {
            let mut longbuffer = vec![];

            for _n in 1..100 {
                let mut buffer = [0; 1];
                let _n = self.serial_port.read(&mut buffer)?;
                let ch = buffer[0];
                longbuffer.push(ch);
                if ch as char == '\n' {
                    break;
                }
            }

            let s = String::from_utf8(longbuffer)?;
            lines.push(s);
        }
        let joined: String = lines.join("");
        Ok(joined)
    }

    fn get_scan_ranges(&mut self) -> Result<Vec<f32>> {
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
                        let range = range_data.parse::<i32>()?;
                        ranges.push((range as f32) / 1000.0); // millimeters to meters
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
    ) -> Result<()> {
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

    fn get_motors(&mut self) -> Result<MotorStatus> {
        log::debug!("get_motors");

        writeln!(self.serial_port, "getmotors\n")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;

        log::debug!("Serial port flushed");

        log::debug!("Reading values...");
        loop {
            let header = self.read_line()?;
            log::debug!("{}", header);

            if header.contains("Parameter,Value") {
                break;
            }
        }

        let lines = self.read_lines(13)?;
        log::debug!("Got {} lines", lines);
        let status = MotorStatus::from_str(&lines)?;
        
        self.motor_status = status;
        log::debug!("Got motors");
        Ok(status)
    }

    fn get_analog_sensors(&mut self) -> Result<AnalogSensorStatus> {
        log::debug!("get_analog_sensors");

        writeln!(self.serial_port, "getanalogsensors\n")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;

        log::debug!("Serial port flushed");

        log::debug!("Reading values...");
        loop {
            let header = self.read_line()?;
            log::debug!("{}", header);

            if header.contains("SensorName,Unit,Value") {
                break;
            }
        }

        let lines = self.read_lines(14)?;
        log::debug!("Got {} lines", lines);
        let status = AnalogSensorStatus::from_str(&lines)?;

        self.analog_sensor_status = status;
        log::debug!("Got analog_sensors");
        Ok(status)
    }

    fn get_digital_sensors(&mut self) -> Result<DigitalSensorStatus> {
        log::debug!("get_digital_sensors");

        writeln!(self.serial_port, "getdigitalsensors\n")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;

        log::debug!("Serial port flushed");
        
        log::debug!("Reading values...");
        loop {
            let header = self.read_line()?;
            log::debug!("{}", header);

            if header.contains("Digital Sensor Name, Value") {
                break;
            }
        }

        let lines = self.read_lines(10)?;
        let status = DigitalSensorStatus::from_str(&lines)?;
        self.digital_sensor_status = status;
        log::debug!("Got digital_sensors");
        Ok(status)
    }

    fn get_charger(&mut self) -> Result<ChargerStatus> {
        log::debug!("get_charger");

        writeln!(self.serial_port, "getcharger\n")?;

        let _s = match self.read_line() {
            Ok(v) => println!("{}", v),
            Err(_) => println!("Error reading back"),
        };

        self.serial_port.flush()?;

        log::debug!("Serial port flushed");

        let mut status = ChargerStatus {
            ..Default::default()
        };

        log::debug!("Reading values...");
        loop {
            let header = self.read_line()?;
            log::debug!("{}", header);

            if header.contains("Label,Value") {
                break;
            }
        }

        for _n in 1..15 {
            // 15 fields
            let s = self.read_line()?;
            log::debug!("{}", s);
            let _ = match IntField::from_str(s.as_str()) {
                Ok(field) => {
                    log::debug!("{:?}", field);
                    match field.name.as_str() {
                        "FuelPercent" => status.fuel_percent = field.value,
                        "BatteryOverTemp" => status.battery_over_tmp = field.value,
                        "ChargingActive" => status.charging_active = field.value,
                        "ChargingEnabled" => status.charging_enabled = field.value,
                        "ConfidentOnFuel" => status.confident_on_fuel = field.value,
                        "OnReservedFuel" => status.on_reserved_fuel = field.value,
                        "EmptyFuel" => status.empty_fuel = field.value,
                        "BatteryFailure" => status.battery_failure = field.value,
                        "ExtPwrPresent" => status.ext_pwr_present = field.value,
                        "ThermistorPresent" => status.thermistor_present = field.value,
                        "BattTempCAvg" => status.batt_temp_c_avg = field.value,
                        "Charger_mAH" => status.charger_mah = field.value,
                        "Discharge_mAH" => status.discharge_mah = field.value,
                        _ => log::error!("Unrecognized field: {:?}", field),
                    }
                }
                Err(_parse_int_error) => {
                    let field = SimpleFloatField::from_str(s.as_str())?;
                    match field.name.as_str() {
                        "VBattV" => status.v_batt_v_v = field.value,
                        "VExtV" => status.v_ext_v = field.value,
                        _ => log::error!("Unrecognized field: {:?}", field),
                    }
                }
            };
        };
        self.charger_status = status;
        log::debug!("Got charger");
        Ok(status)
    }

    fn set_backlight(&mut self, value: Toggle) -> Result<()> {
        writeln!(self.serial_port, "setled backlight{}", value.to_string())?;
        Ok(())
    }
}
