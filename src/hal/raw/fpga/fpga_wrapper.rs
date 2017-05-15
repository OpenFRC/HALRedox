//use ::fpga::fpga_constants;
use ::fpga::fpga_types;

//use std::os::raw::{c_char, c_float, c_double};

pub type tRioStatusCode = fpga_types::NiFpga_Status;
pub type RioStatusPointer = *mut fpga_types::NiFpga_Status;


pub mod system {

    extern {

    }

}
