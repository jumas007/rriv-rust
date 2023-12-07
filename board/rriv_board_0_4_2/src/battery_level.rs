use crate::*;

pub struct BatteryLevel {
  pins: pin_groups::BatteryLevelPins
}

impl BatteryLevel {
  pub fn new(pins: pin_groups::BatteryLevelPins) -> Self {
    return BatteryLevel {
      pins
    }
  }

  pub fn measure_battery_level(&self, adc: &InternalAdc, delay: &SysDelay) -> Result<u16, AdcError> {

    self.pins.enable_vin_measure.set_low();
    delay.delay_ms(1000_u16);
    let value = adc.read_battery_level();
    self.pins.enable_vin_measure.set_high();
    value

  }
}

