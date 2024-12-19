.PHONY: all sp1 risc0 jolt nexus ceno clean

sp1-fib:
	cd sp1/examples/fibonacci/program && cargo prove build && \
	cd ../script && gtime -v cargo run --release > ../../../../results/sp1_fibonacci.log 2>&1

risc0-fib:
	cd risc0/benchmarks && gtime -v cargo run --release -- fibonacci > ../../results/risc0_fibonacci.log 2>&1 && \
	mv metrics.csv ../../results/risc0_fibonacci_metrics.csv

risc0-sha2:
	cd risc0/benchmarks && gtime -v cargo run --release -- ecdsa-verify > ../../results/risc0_ecdsa_verify.log 2>&1 && \
	mv metrics.csv ../../results/risc0_ecdsa_verify_metrics.csv

risc0-matrix-ops:
	cd risc0/benchmarks && gtime -v cargo run --release -- fibonacci > ../../results/risc0_matrix_ops.log 2>&1 && \
	mv metrics.csv ../../results/risc0_matrix_ops_metrics.csv

jolt-fib:
	cd jolt && cargo run -p jolt-core --release -- trace --name fibonacci --format chrome --pcs hyper-kzg > ../results/jolt_fibonacci.log 2>&1 && \
	mv trace-*.json ../results/trace-fibonacci.json

nexus-fib:
	cd nexus/examples && RUST_LOG=debug cargo run -r --bin fib3 > ../../results/nexus_fibonacci.log 2>&1

ceno-fib:
	cd ceno/ceno_zkvm && cargo bench fibonacci > ../../results/ceno_fibonacci.log 2>&1

clean:
	cd sp1 && cargo clean
	cd risc0 && cargo clean
	cd jolt && cargo clean
	cd nexus && cargo clean
	cd ceno && cargo clean
