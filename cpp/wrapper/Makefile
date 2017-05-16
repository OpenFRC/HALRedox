default:
	g++ -c -o ../../lib/ni-wrapper/tPWM.o src/tPWM.cpp -I.
	g++ -c -o ../../lib/ni-wrapper/tDIO.o src/tDIO.cpp -I.
	g++ -c -o ../../lib/ni-wrapper/tRelay.o src/tRelay.cpp -I.
	ar cr ../../lib/ni-wrapper/ni-wrapper.a ../../lib/ni-wrapper/*

arm:
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tPWM.o src/tPWM.cpp -I.
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tDIO.o src/tDIO.cpp -I.
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tRelay.o src/tRelay.cpp -I.
	ar cr ../../lib/ni-wrapper/ni-wrapper.a ../../lib/ni-wrapper/*

clean:
	rm -rf ../../lib/ni-wrapper/*
