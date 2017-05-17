#include <memory>

#include "include/nRoboRIO_FPGANamespace/tSPI.h"

using namespace nFPGA;
using namespace nFPGA::nFRC_2017_17_0_2;

extern "C" {

    static tSPI* tSPI__create(tRioStatusCode *status) {
        return tSPI::create(status);
    }
}
