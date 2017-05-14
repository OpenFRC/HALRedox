use hal::commands::*;
use ni::raw::pwm;
use ni::raw::fpga_types;
use ni::raw::fpga_constants;
use ni::raw::fpga_wrapper;

use std::os::raw::{c_void, c_ushort, c_uchar};


struct PWMCreate;

impl commands::Command for PWMCreate {
    type Output = *pwm::tPWM;

    fn execute(self, _: &mut commands::HardwareContext) -> Result<u32> {
        Ok(pwm::tPWM__create(fpga_constants::NiFpga_Status_Success));
    }
}

struct PWMWriteConfigPeriod {
    value: c_ushort,
    tPWM: pwm::PWMPointer;
}

impl commands::CommandCommandFuturefor PWMWriteConfigPeriod {
    type Output = fpga_wrapper::RioStatusPointer;
    fn execute(self, _: &mut commands::HardwareContext) -> Result<u32> {
        Ok(pwm::writeConfig_Period{self.value, self.tPWM})
    }
}


struct PWMWriteConfigMinHigh{
    value: c_ushort,
    tPWM: pwm::PWMPointer;
}

impl commands::Command for PWMWriteConfigPeriod {
    type Output = fpga_wrapper::RioStatusPointer;
    fn execute(self, _: &mut commands::HardwareContext) -> Result<u32> {
        Ok(pwm::writeConfig_MinHigh{self.value, self.tPWM})
    }
}

struct PWM {
    pwm_reference: *tPWM,
    sender: command::CommandSender,
}
impl PWM {
    pub fn new(input_sender: command::CommandSender) -> PWM {
        let pwm_future = sender.run(PWMCreate).unwrap;
        temp_pwm = pwm_future.wait();
        return PWM {
            pwm_reference: temp_pwm,
            sender: input_sender,
        }
    }
    pub fn writeConfigPeriod(value: c_ushort) -> fpga_types::NiFpga_Status {
        let status_future = sender.run(PWMWriteConfigPeriod(self.value, self.pwm_reference));
        return status_future.wait();
    }
}

#[cfg(test)]
mod test {
    use hal::commands::*;
    use ni::raw::pwm;
    use ni::raw::fpga_types;
    use ni::raw::fpga_constants;

    use std::os::raw::{c_void, c_ushort, c_uchar};
    use ni::wrapper::pwm::*

    #[test]
    fn compile_test() {
        let sender = command::spawn_hardware_thread();
        let future = sender.run(PWMCreate).unwrap();
        assert!(future.wait().is_err());
    }
}
