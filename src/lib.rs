use serialport::{posix::TTYPort, SerialPort};

#[derive(Debug)]
pub enum Toggle {
    On,
    Off,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    fn exit(&self);
    fn set_testmode(&self, value: Toggle);
    fn set_ldsrotation(&self, value: Toggle);

    fn request_scan(&self);
    fn get_scan_ranges(&self) -> Vec<f32>;

    fn set_motors(&self, left_distance: i32, right_distance: i32, speed: i32);
    fn get_motors(&self) -> MotorStatus;

    fn get_analog_sensors(&self) -> AnalogSensorStatus;
    fn get_digital_sensors(&self) -> DigitalSensorStatus;
    fn get_charger(&self) -> ChargerStatus;

    fn set_backlight(&self, value: Toggle);
}

// impl std::fmt::Debug for dyn SerialPort {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self.name() {
//             Some(x) => write!(f, "{}", x),
//             None    => write!(f, "None"),
//         }
//     }
// }

pub struct DSeries {
    serial_port: Box<dyn SerialPort>,
    motor_status: MotorStatus,
    analog_sensor_status: AnalogSensorStatus,
    digital_sensor_status: DigitalSensorStatus,
    charger_status: ChargerStatus,
}

impl NeatoRobot for DSeries {
    fn exit(&self) {
        self.set_ldsrotation(Toggle::Off);
        self.set_testmode(Toggle::Off);
    }

    fn set_testmode(&self, value: Toggle) {
        todo!()
    }

    fn set_ldsrotation(&self, value: Toggle) {
        todo!()
    }

    fn request_scan(&self) {
        todo!()
    }

    fn get_scan_ranges(&self) -> Vec<f32> {
        todo!()
    }

    fn set_motors(&self, left_distance: i32, right_distance: i32, speed: i32) {
        todo!()
    }

    fn get_motors(&self) -> MotorStatus {
        todo!()
    }

    fn get_analog_sensors(&self) -> AnalogSensorStatus {
        todo!()
    }

    fn get_digital_sensors(&self) -> DigitalSensorStatus {
        todo!()
    }

    fn get_charger(&self) -> ChargerStatus {
        todo!()
    }

    fn set_backlight(&self, value: Toggle) {
        todo!()
    }
}
