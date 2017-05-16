use hal::commands::*;
use hal::raw::{PwmPointer};
use hal::raw::fpga_types;
use hal::raw::fpga_constants;
use hal::raw::fpga_wrapper::{RioStatusPointer}

use std::os::raw::{c_void, c_ushort, c_uchar};


struct PwmCreate;

impl Command for PwmCreate {
    type Output = pwm::PwmPointer;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(unsafe{pwm::tPwm__create(fpga_constants::NiFpga_Status_Success)});
    }
}

struct PwmWriteConfigPeriod {
    value: c_ushort,
    pwm_object: PwmPointer;
}

impl Command for PwmWriteConfigPeriod {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writeConfig_Period(self.value, self.tPwm))
    }
}


struct PwmWriteConfigMinHigh {
    value: c_ushort,
    tPwm: PwmPointer;
}

impl Command for PwmWriteConfigPeriod {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writeConfig_MinHigh(self.value, self.tPwm))
    }
}

struct PwmReadLoopTiming {
    tPwm: PwmPointer;
}

impl Command for PwmReadLoopTiming {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::readLoopTiming(self.tPwm))
    }
}

struct PwmWriteHdr {
    reg_index: c_uchar,
    value: c_ushort,
    tPwm: PwmPointer;
}

impl Command for PwmWriteHdr {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writeHdr(self.reg_index, self.value, self.tPwm))
    }
}

struct PwmWriteMXP {
    reg_index: c_uchar,
    value: c_ushort,
    tPwm: PwmPointer;
}

impl Command for PwmWriteHdr {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writeMXP(self.reg_index, self.value, self.tPwm))
    }
}

struct PwmWritePeriodScaleHdr {
    bitfield_index: c_uchar,
    value: c_ushort,
    tPwm: PwmPointer;
}

impl Command for PwmWritePeriodScaleHdr {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writePeriodScaleHdr(self.bitfield_index, self.value, self.tPwm))
    }
}

struct PwmWritePeriodScaleMXP {
    bitfield_index: c_uchar,
    value: c_ushort,
    tPwm: PwmPointer;
}

impl Command for PwmWritePeriodScaleMXP {
    type Output = RioStatusPointer;
    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(pwm::writePeriodScaleMXP(self.bitfield_index, self.value, self.tPwm))
    }
}

#[cfg(test)]
mod test {
    use hal::commands::*;
    use ::pwm;
    use ::fpga_types;
    use ::fpga_constants;

    use std::os::raw::{c_void, c_ushort, c_uchar};
    use wrapper::pwm::*

    #[test]
    fn compile_test() {
        let sender = command::spawn_hardware_thread();
        let future = sender.run(PwmCreate).unwrap();
        assert!(future.wait().is_err());
    }
}
