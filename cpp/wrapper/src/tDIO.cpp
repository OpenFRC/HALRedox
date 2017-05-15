#include "include/nRoboRIO_FPGANamespace/tDIO.h"

using nFPGA;
using nFPGA::nFRC_2017_17_0_2;

extern "C" {

    static tDIO* tDIO__create(tRioSTatusCode *status) {
        return tDIO::create(status);
    }
}
