use hal::commands::*;
use hal::raw::{SPIPointer};
use hal::raw::fpga_types;
use hal::raw::fpga_constants;
use hal::raw::fpga_wrapper::{RioStatusPointer}

use std::os::raw::{c_void, c_ushort, c_uchar};

struct SPICreate;

impl Command for SPICreate {
    type Output = *SPI::tSPI;

    fn execute(self, _: &mut HardwareContext) -> Result<u32> {
        Ok(unsafe{SPI::tSPI__create(fpga_constants::NiFpga_Status_Success)});
    }
}
