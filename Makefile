.PHONY: all sp1 risc0 jolt nexus ceno clean

all: sp1 risc0 jolt nexus ceno

sp1:
	cd sp1/examples/fibonacci/program && cargo prove build && \
	cd ../script && cargo run --release > ../../../../results/sp1_fibonacci.log 2>&1

risc0:
	cd risc0/benchmarks && cargo run --release -- fibonacci > ../../results/risc0_fibonacci.log 2>&1

jolt:
	cd jolt && cargo run -p jolt-core --release -- trace --name fibonacci --format chrome --pcs hyper-kzg > ../results/jolt_fibonacci.log 2>&1 && \
	mv trace-*.json ../results/trace-fibonacci.json

nexus:
	cd nexus/examples && RUST_LOG=debug cargo run -r --bin fib3 > ../../results/nexus_fibonacci.log 2>&1

ceno:
	cd ceno/ceno_zkvm && cargo bench fibonacci > ../../results/ceno_fibonacci.log 2>&1

clean:
	cd sp1 && cargo clean
	cd risc0 && cargo clean
	cd jolt && cargo clean
	cd nexus && cargo clean
	cd ceno && cargo clean
