use hal::commands::*;
use hal::raw::{PwmPointer};
use hal::raw::fpga_types;
use hal::raw::fpga_constants;
use hal::raw::fpga_wrapper::{RioStatusPointer}

use std::os::raw::{c_void, c_ushort, c_uchar};

struct RelayCreate;

impl Command for RelayCreate {
    type Output = relay::RelayPointer;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(unsafe{tRelay__create(fpga_constants::NiFpga_Status_Success)});
    }
}

struct RelayWriteValueForward {
    value: c_uchar,
    relay_object: RelayPointer,
}

impl Command for RelayWriteValueForward {
    type Output = c_void;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok{relay::writeValue_Forward(self.value, self.relay_object)}
    }
}

struct RelayWriteValueReverse {
    value: c_uchar,
    relay_object: RelayPointer,
}

impl Command for RelayWriteValueReverse {
    type Output = c_void;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok{relay::writeValue_Reverse(self.value, self.relay_object)}
    }
}
