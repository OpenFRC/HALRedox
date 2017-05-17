default:
	rm -rf ../../lib/ni-wrapper
	mkdir ../../lib/ni-wrapper
	g++ -c -o ../../lib/ni-wrapper/tPWM.o src/tPWM.cpp -I.
	g++ -c -o ../../lib/ni-wrapper/tDIO.o src/tDIO.cpp -I.
	g++ -c -o ../../lib/ni-wrapper/tRelay.o src/tRelay.cpp -I.
	ar cr ../../lib/ni-wrapper/ni-wrapper.a ../../lib/ni-wrapper/*

arm:
	rm -rf ../../lib/ni-wrapper
	mkdir ../../lib/ni-wrapper
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tPWM.o src/tPWM.cpp -I.
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tDIO.o src/tDIO.cpp -I.
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/tRelay.o src/tRelay.cpp -I.
	ar cr ../../lib/ni-wrapper/ni-wrapper.a ../../lib/ni-wrapper/*

clean:
	rm -rf ../../lib/ni-wrapper/*
