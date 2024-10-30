target: build
.PHONY: build run release clean distclean

build:
	@ cargo build

run:
	@ cargo run

release:
	@ cargo build --release --target=x86_64-unknown-linux-gnu

clean:
	@ cargo clean

distclean: clean
	@ rm -rf out/ __pycache__/

DEMO_BINS = demo_duplex64 demo_duplex256 demo_gt2016_64 demo_gt2016_256
.PHONY: $(DEMO_BINS)

bin_%:
	@echo "Running demo $*..."
	@cargo run --bin $*

$(foreach bin,$(DEMO_BINS),$(eval $(bin): bin_$(bin)))