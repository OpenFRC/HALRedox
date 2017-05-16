use hal::commands::*;
use hal::raw::{DioPointer};
use hal::raw::fpga_types;
use hal::raw::fpga_constants;
use hal::raw::fpga_wrapper::{RioStatusPointer}

use std::os::raw::{c_void, c_ushort, c_uchar};

struct DioCreate;

impl Command for DioCreate {
    type Output = *dio::tDio;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(unsafe{dio::tDio__create(fpga_constants::NiFpga_Status_Success)});
    }
}
