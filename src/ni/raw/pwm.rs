use std::os::raw::{c_void, c_ushort, c_uchar};

//#[repr(c_void)]
struct tPWM;

extern {
	fn tPWM__new() -> *mut tPWM;
	fn tPWM__delete(tPWM: *mut tPWM);
	
	fn tPWM__create(status: *mut tRioStatusCode) -> *mut tPWM;
	
	fn tPWM__writeConfig_Period(value: c_ushort, status: *mut tRioStatusCode, This: *mut tPWM);
	fn tPWM__writeConfig_MinHigh(value: c_ushort, status: *mut tRioStatusCode, This: *mut tPWM);
	
	fn tPWM__readLoopTiming(status: *mut tRioStatusCode, This: *mut tPWM) -> c_ushort;
	
	fn tPWM__writeHdr(reg_index: c_uchar, value: c_ushort, , status: *mut tRioStatusCode, This: *mut tPWM);
	fn tPWM__writeMXP(reg_index: c_uchar, value: c_ushort, , status: *mut tRioStatusCode, This: *mut tPWM); 

	fn tPWM__writePeriodScaleHdr(bitfield_index: c_uchar, value: c_ushort, , status: *mut tRioStatusCode, This: *mut tPWM);
	fn tPWM__writePeriodScaleMXP(bitfield_index: c_uchar, value: c_ushort, , status: *mut tRioStatusCode, This: *mut tPWM);
}