use std::os::raw::{c_void, c_ushort, c_uchar};
use hal::raw::fpga::fpga_wrapper::RioStatusPointer;
use hal::raw::fpga::fpga_wrapper::tRioStatusCode;
use hal::raw::fpga::fpga_types;

//#[repr(c_void)]
struct tPwm;

pub struct PwmPointer(*mut tPWM);

extern {
	pub fn tPWM__new() -> PwmPointer;
	pub fn tPWM__delete(tPWM: PwmPointer);

	pub fn tPWM__create(status: RioStatusPointer) -> PwmPointer;

	pub fn tPWM__writeConfig_Period(value: c_ushort, status: RioStatusPointer, this: PwmPointer);
	pub fn tPWM__writeConfig_MinHigh(value: c_ushort, status: RioStatusPointer, this: PwmPointer);

	pub fn tPWM__readLoopTiming(status: RioStatusPointer, this: PwmPointer) -> c_ushort;

	pub fn tPWM__writeHdr(reg_index: c_uchar, value: c_ushort, status: RioStatusPointer, this: PwmPointer);
	pub fn tPWM__writeMXP(reg_index: c_uchar, value: c_ushort, status: RioStatusPointer, this: PwmPointer);

	pub fn tPWM__writePeriodScaleHdr(bitfield_index: c_uchar, value: c_ushort, status: RioStatusPointer, this: PwmPointer);
	pub fn tPWM__writePeriodScaleMXP(bitfield_index: c_uchar, value: c_ushort, status: RioStatusPointer, this: PwmPointer);
}

pub fn writeConfig_Period(value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writeConfig_Period(value, status, this);}
	return status;

}
pub fn writeConfig_MinHigh(value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writeConfig_MinHigh(value, status, this);}
	return status;

}

pub fn readLoopTiming(this: PwmPointer) -> RioStatusPointer {
	let mut sucess_code: tRioStatusCode = 0;
	let status: RioStatusPointer= &mut sucess_code;
	unsafe{tPWM__readLoopTiming(this);}
	return status;
}

pub fn writeHdr(reg_index: c_uchar, value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writeHdr(reg_index, value, status, this);}
	return status;

}
pub fn writeMXP(reg_index: c_uchar, value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writeMXP(reg_index, value, status, this);}
	return status;

}
pub fn writePeriodScaleHdr(bitfield_index: c_uchar, value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writePeriodScaleHdr(bitfield_index, value, status, this);}
	return status;

}
pub fn writePeriodScaleMXP(reg_index: c_uchar, value: c_ushort, this: PwmPointer) -> RioStatusPointer {
	let mut success_code: tRioStatusCode = 0;
	let status: RioStatusPointer = &mut success_code;
	unsafe{tPWM__writePeriodScaleMXP(bitfield_index, value, status, this);}
	return status;

}
