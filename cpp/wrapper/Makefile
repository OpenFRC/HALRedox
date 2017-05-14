default:
	g++ -c -o ../../lib/ni-wrapper/ni_wrapper.a src/tPWM.cpp -I.

arm:
	mkdir ../../lib/ni-wrapper
	arm-frc-linux-gnueabi-g++ -c -o ../../lib/ni-wrapper/ni_wrapper.a src/tPWM.cpp -I.

clean:
	rm -rf ../../lib/ni-wrapper/*
