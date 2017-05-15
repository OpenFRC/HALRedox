use std::os::raw::{c_void, c_ushort, c_uchar};
use hal::raw::fpga::fpga_wrapper::RioStatusPointer;
use hal::raw::fpga::fpga_wrapper::tRioStatusCode;
use hal::raw::fpga::fpga_types;

//#[repr(c_void)]
struct tDio;

pub struct DioPointer(*mut tDio);


extern {
    pub fn tDIO__create(status: RioStatusPointer) -> DioPointer;
}
