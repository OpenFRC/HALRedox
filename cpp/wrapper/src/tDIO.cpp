#include <memory>

#include "include/nRoboRIO_FPGANamespace/tDIO.h"

using namespace nFPGA;
using namespace nFPGA::nFRC_2017_17_0_2;

extern "C" {

    static tDIO* tDIO__create(tRioStatusCode *status) {
        return tDIO::create(status);
    }
}
