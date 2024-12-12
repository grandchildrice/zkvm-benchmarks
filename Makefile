.PHONY: all sp1 risc0 jolt nexus ceno clean

all: sp1 risc0 jolt nexus ceno

sp1:
	cd sp1 && cargo run --example example_name

risc0:
	cd risc0 && cargo run --example example_name

jolt:
	cd jolt && cargo run --example example_name

nexus:
	cd nexus && cargo run --example example_name

ceno:
	cd ceno && cargo run --example example_name

clean:
	cd sp1 && cargo clean
	cd risc0 && cargo clean
	cd jolt && cargo clean
	cd nexus && cargo clean
	cd ceno && cargo clean

