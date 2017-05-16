#include <memory>

#include "include/nRoboRIO_FPGANamespace/tRelay.h"

using namespace nFPGA;
using namespace nFPGA::nFRC_2017_17_0_2;

extern "C" {

    static tRelay* tRelay__create(tRioStatusCode *status) {
        return tRelay::create(status);
    }
    void tRelay__writeValue_Forward(unsigned char value, tRioStatusCode *status, tRelay *This) {
        This->writeValue_Forward(value, status);
    }
    void tRelay__writeValue_Reverse(unsigned char value, tRioStatusCode *status, tRelay *This) {
        This->writeValue_Reverse(value, status);
    }
}
