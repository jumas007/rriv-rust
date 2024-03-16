
// codes and sensor names mapped to a sensor implementation

use rriv_board::EEPROM_SENSOR_SETTINGS_SIZE;


pub const SENSOR_SETTINGS_PARTITION_SIZE: usize = 32; // partitioning is part of the driver implemention, and not meaningful at the EEPROM level
pub type SensorGeneralSettingsSlice = [u8; SENSOR_SETTINGS_PARTITION_SIZE];
pub type SensorSpecialSettingsSlice = [u8; SENSOR_SETTINGS_PARTITION_SIZE];


#[derive(Copy,Clone,Debug)]
pub struct SensorDriverGeneralConfiguration {
    pub id: [u8;6],
    pub sensor_type_id: u16,
    pub warmup: u16,
    pub burst_repetitions: u8
}

impl SensorDriverGeneralConfiguration {
    pub fn new ( 
        id: [u8;6],
        sensor_type_id: u16,
     ) -> SensorDriverGeneralConfiguration{
            Self {
                id: id,
                sensor_type_id: sensor_type_id,
                warmup: 0,
                burst_repetitions: 1
            }
        }

    pub fn new_from_bytes(bytes: &[u8; SENSOR_SETTINGS_PARTITION_SIZE] ) -> SensorDriverGeneralConfiguration {
        let settings = bytes.as_ptr().cast::<SensorDriverGeneralConfiguration>();
        unsafe {
            *settings
        }
    }
}


pub trait SensorDriver {
    fn setup(&mut self);
    fn get_id(&mut self) -> [u8;6];
    fn get_type_id(&mut self) -> u16;
}

pub trait ActuatorDriver {
    fn setup(&mut self);
}

pub trait TelemeterDriver {
    fn setup(&mut self);
}

#[derive(Copy,Clone,Debug)]
pub struct GenericAnalogSpecialConfiguration {
    m: f64, //8
    b: f64, // 8
    sensor_port: u8, // 1
    empty: [u8; 15] // 15
}

impl GenericAnalogSpecialConfiguration {
    pub fn new_from_values(value: serde_json::Value) -> GenericAnalogSpecialConfiguration { // should we return a Result object here? because we are parsing?  parse_from_values?
        let mut sensor_port: u8 = 0;
        match &value["sensor_port"] {
            serde_json::Value::Number(number) => {
                if let Some(number) = number.as_u64() {
                    let number: Result<u8, _> = number.try_into();
                    match number {
                        Ok(number) => {
                            sensor_port = number;
                        }
                        Err(_) => todo!("need to handle invalid number"),

                    }
                }
            }
            _ => {todo!("need to handle missing sensor port")},
        }

        return Self {
            m: 0.0,
            b: 0.0,
            sensor_port: sensor_port,
            empty: [b'\0'; 15]
        }
    }

    pub fn new_from_bytes(bytes: [u8; SENSOR_SETTINGS_PARTITION_SIZE] ) -> GenericAnalogSpecialConfiguration {
        // panic if bytes.len() != 32
        let settings = bytes.as_ptr().cast::<GenericAnalogSpecialConfiguration>();
        unsafe {
            *settings
        }
    }
}


macro_rules! getters {
    () => {
        fn get_id(&mut self) -> [u8;6] {
            self.general_config.id.clone()
        } 

        fn get_type_id(&mut self) -> u16 {
            self.general_config.sensor_type_id.clone()
        } 
    };
}


pub struct GenericAnalog {
    general_config: SensorDriverGeneralConfiguration,
    special_config: GenericAnalogSpecialConfiguration
}   

impl SensorDriver for GenericAnalog {
    fn setup(&mut self) {
        todo!()
    }

    getters!();
}

impl GenericAnalog {
    // pub fn new(general_config: SensorDriverGeneralConfiguration, special_config_bytes: &[u8; rriv_board::EEPROM_SENSOR_SPECIAL_SETTINGS_SIZE]) -> Self {
        
    //     let special_config = GenericAnalogSpecialConfiguration::new_from_bytes(special_config_bytes);
    //     GenericAnalog {
    //         general_config,
    //         special_config
    //     }
    // }

    pub fn new(general_config: SensorDriverGeneralConfiguration, special_config: GenericAnalogSpecialConfiguration) -> Self  {
        GenericAnalog {
            general_config,
            special_config
        }
    }
}


#[derive(Copy,Clone,Debug)]
pub struct AHT22SpecialConfiguration {
    wait_time : usize,
    empty: [u8; 28]
}

impl AHT22SpecialConfiguration {
    pub fn new_from_bytes(bytes: &[u8; SENSOR_SETTINGS_PARTITION_SIZE] ) -> AHT22SpecialConfiguration {
        let settings = bytes.as_ptr().cast::<AHT22SpecialConfiguration>();
        unsafe {
            *settings
        }
    }
}


pub struct AHT22 {
    general_config: SensorDriverGeneralConfiguration,
    special_config: AHT22SpecialConfiguration
}

impl SensorDriver for AHT22 {
    fn setup(&mut self) {
        todo!()
    }

    getters!();
}

impl AHT22 {
    pub fn new(general_config: SensorDriverGeneralConfiguration, specific_config_bytes: &[u8; SENSOR_SETTINGS_PARTITION_SIZE]) -> Self {
        
        let special_config = AHT22SpecialConfiguration::new_from_bytes(specific_config_bytes);
        AHT22 {
            general_config,
            special_config
        }
    }
}