use std::os::raw::{c_void, c_ushort, c_uchar};
use hal::raw::fpga::fpga_wrapper::RioStatusPointer;
use hal::raw::fpga::fpga_wrapper::tRioStatusCode;
use hal::raw::fpga::fpga_types;

//#[repr(c_void)]
struct tSPI;

pub struct SPIPointer(*mut tSPI);


extern {
    pub fn tSPI__create(status: RioStatusPointer) -> SPIPointer;
}
