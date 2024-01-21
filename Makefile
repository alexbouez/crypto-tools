OUT = out/plots/ out/csv/ 
OPTIONS = distrib convolution fourier hadamard 

.PHONY: out build run clean distclean 

target: run
run: 
	@ cargo run
build: 
	@ cargo build
release:
	@ cargo build --release --target=x86_64-unknown-linux-gnu

clean: 
	@ cargo clean
distclean: clean
	@ rm -rf out/ __pycache__/

.PHONY: sprng64 sprng512

sprng64: 
	@ cargo run --bin demo_sprng64
sprng512: 
	@ cargo run --bin demo_sprng512