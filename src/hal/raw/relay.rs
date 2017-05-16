use std::os::raw::{c_void, c_ushort, c_uchar};
use hal::raw::fpga::fpga_wrapper::RioStatusPointer;
use hal::raw::fpga::fpga_wrapper::tRioStatusCode;
use hal::raw::fpga::fpga_types;

//#[repr(c_void)]
struct tRelay;

pub struct RelayPointer(*mut tRelay);

extern {
    pub fn tRelay__create(status: RioStatusPointer) -> RelayPointer;

    pub fn tRelay__writeValue_Forward(value: c_uchar, status: RioStatusPointer, this: RelayPointer);
    pub fn tRelay__writeValue_Reverse(value: c_uchar, status: RioStatusPointer, this: RelayPointer);

}

pub fn writeValue_Forward(value: c_uchar, this: RelayPointer) {
    let mut success_code: tRioStatusCode = 0;
    let status: RioStatusPointer = &mut success_code;
    unsafe{tRelay__writeValue_Forward(value, status, this);}
    return status;
}

pub fn writeValue_Reverse(value: c_uchar, this: RelayPointer) {
    let mut success_code: tRioStatusCode = 0;
    let status: RioStatusPointer = &mut success_code;
    unsafe{tRelay__writeValue_Reverse(value, status, this);}
    return status;
}
