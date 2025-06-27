target: build
.PHONY: build run release clean distclean

build:
	@ cargo build

run:
	@ cargo run

release:
	@ cargo build --release --target=x86_64-unknown-linux-gnu

doc:
	@ cargo doc --no-deps --open

clean:
	@ cargo clean

distclean: clean
	@ rm -rf out/ __pycache__/

DEMO_BINS = asakey_256 dss_256 duplex_64 duplex_256 sprng_64 sprng_256
.PHONY: $(DEMO_BINS)

bin_%:
	@echo "Running demo $*..."
	@cargo run --bin $*

$(foreach bin,$(DEMO_BINS),$(eval $(bin): bin_$(bin)))