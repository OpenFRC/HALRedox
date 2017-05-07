#include <memory>

#include "include/nRoboRIO_FPGANamespace/tPWM.h"

using namespace nFPGA;
using namespace nFRC_2017_17_0_2;

extern "C" {

	static tPWM* tPWM__create(tRioStatusCode *status) {
		return tPWM::create(status);
	}

	void tPWM__delete(tPWM *This) {
		delete This;
	}

	void tPWM__writeConfig_Period(unsigned short value, tRioStatusCode *status, tPWM *This) {
		This->writeConfig_Period(value, status);
	}

	void tPWM__writeConfig_MinHigh(unsigned short value, tRioStatusCode *status, tPWM *This) {
			This->writeConfig_MinHigh(value, status);
	}

	unsigned short tPWM__readLoopTiming(tRioStatusCode *status, tPWM *This) {
		return This->readLoopTiming(status);
	}


	void tPWM__writeHdr(unsigned char reg_index, unsigned short value, tRioStatusCode *status, tPWM *This) {
		This->writeHdr(reg_index, value, status);
	}

	void tPWM__writeMXP(unsigned char reg_index, unsigned short value, tRioStatusCode *status, tPWM *This) {
		This->writeMXP(reg_index, value, status);
	}

	void tPWM__writePeriodScaleHdr(unsigned char bitfield_index, unsigned char value, tRioStatusCode *status, tPWM  *This) {
		This->writePeriodScaleHdr(bitfield_index, value, status);
	}

	void tPWM__writePeriodScaleMXP(unsigned char bitfield_index, unsigned char value, tRioStatusCode *status, tPWM *This) {
		This->writePeriodScaleMXP(bitfield_index, value, status);
	}
}
