.PHONY: all sp1 risc0 jolt nexus ceno clean

sp1-fib:
	cd sp1_main/examples/fibonacci/program && cargo prove build && \
	cd ../script && \
	gtime -v cargo run --release -- 10 > ../../../../results/sp1_fibonacci_n10.log 2>&1 && \
	gtime -v cargo run --release -- 100 > ../../../../results/sp1_fibonacci_n100.log 2>&1 && \
	gtime -v cargo run --release -- 1000 > ../../../../results/sp1_fibonacci_n1000.log 2>&1 && \
	gtime -v cargo run --release -- 10000 > ../../../../results/sp1_fibonacci_n10000.log 2>&1 && \
	gtime -v cargo run --release -- 100000 > ../../../../results/sp1_fibonacci_n100000.log 2>&1 && \
	gtime -v cargo run --release -- 1000000 > ../../../../results/sp1_fibonacci_n1000000.log 2>&1

sp1-matrix-ops:
	cd sp1_main/examples/matrix-ops/program && cargo prove build && \
	cd ../script && \
	gtime -v cargo run --release -- 10 > ../../../../results/sp1_matrix-ops_n10.log 2>&1 && \
	gtime -v cargo run --release -- 20 > ../../../../results/sp1_matrix-ops_n20.log 2>&1 && \
	gtime -v cargo run --release -- 30 > ../../../../results/sp1_matrix-ops_n30.log 2>&1 && \
	gtime -v cargo run --release -- 40 > ../../../../results/sp1_matrix-ops_n40.log 2>&1 && \
	gtime -v cargo run --release -- 50 > ../../../../results/sp1_matrix-ops_n50.log 2>&1

risc0-fib:
	cd risc0/benchmarks && gtime -v cargo run --release -- fibonacci > ../../results/risc0_fibonacci.log 2>&1 && \
	mv metrics.csv ../../results/risc0_fibonacci_metrics.csv

risc0-sha2:
	cd risc0/benchmarks && gtime -v cargo run --release -- ecdsa-verify > ../../results/risc0_ecdsa_verify.log 2>&1 && \
	mv metrics.csv ../../results/risc0_ecdsa_verify_metrics.csv

risc0-matrix-ops:
	cd risc0/benchmarks && gtime -v cargo run --release -- iter-blake2b > ../../results/risc0_matrix_ops.log 2>&1 && \
	mv metrics.csv ../../results/risc0_matrix_ops_metrics.csv

jolt-fib:
	cd jolt && \
	gtime -v cargo run --release -p fibonacci -- 10 > ../results/jolt_fibonacci_n10.log 2>&1 && \
	gtime -v cargo run --release -p fibonacci -- 100 > ../results/jolt_fibonacci_n100.log 2>&1 && \
	gtime -v cargo run --release -p fibonacci -- 1000 > ../results/jolt_fibonacci_n1000.log 2>&1 && \
	gtime -v cargo run --release -p fibonacci -- 10000 > ../results/jolt_fibonacci_n10000.log 2>&1 && \
	gtime -v cargo run --release -p fibonacci -- 100000 > ../results/jolt_fibonacci_n100000.log 2>&1

jolt-matrix-ops:
	cd jolt && \
	gtime -v cargo run --release -p matrix-ops -- 10 > ../results/jolt_matrix_ops_n10.log 2>&1 && \
	gtime -v cargo run --release -p matrix-ops -- 20 > ../results/jolt_matrix_ops_n20.log 2>&1 && \
	gtime -v cargo run --release -p matrix-ops -- 30 > ../results/jolt_matrix_ops_n30.log 2>&1 && \
	gtime -v cargo run --release -p matrix-ops -- 40 > ../results/jolt_matrix_ops_n40.log 2>&1 && \
	gtime -v cargo run --release -p matrix-ops -- 50 > ../results/jolt_matrix_ops_n50.log 2>&1

jolt-profile:
	cd jolt && cargo run -p jolt-core --release -- trace --name fibonacci --format chrome --pcs hyper-kzg > ../results/jolt_profile.log 2>&1 && \
	mv trace-*.json ../results/trace-fibonacci.json

nexus-fib:
	cd nexus/examples && \
	gtime -v cargo nexus prove --bin fib10 > ../../results/nexus_fibonacci_n10.log 2>&1 && \
	gtime -v cargo nexus prove --bin fib100 > ../../results/nexus_fibonacci_n100.log 2>&1 && \
	gtime -v cargo nexus prove --bin fib1000 > ../../results/nexus_fibonacci_n1000.log 2>&1 && \
	gtime -v cargo nexus prove --bin fib10000 > ../../results/nexus_fibonacci_n10000.log 2>&1 && \
	gtime -v cargo nexus prove --bin fib100000 > ../../results/nexus_fibonacci_n100000.log 2>&1 && \
	gtime -v cargo nexus prove --bin fib1000000 > ../../results/nexus_fibonacci_n1000000.log 2>&1 && 

ceno-fib:
	cd ceno/ceno_zkvm && gtime -v cargo bench fibonacci > ../../results/ceno_fibonacci.log 2>&1

clean:
	cd sp1 && cargo clean
	cd risc0 && cargo clean
	cd jolt && cargo clean
	cd nexus && cargo clean
	cd ceno && cargo clean
