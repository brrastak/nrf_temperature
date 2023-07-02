
flash: build
	cargo flash --release --chip nRF51822_xxAA

build:
	cargo build --release
	cargo size --release
	
size:
	cargo size --release

rtt:
	cargo embed --release
