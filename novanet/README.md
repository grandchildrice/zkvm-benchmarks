```bash
RUST_LOG=debug cargo run --release -p runner --  --guest "ethblock" --benchmark-args "<number of transactions>", "<number of transactions>",...
```
Examples
```bash
RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 16 32
```
```bash
RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 16 32 --compress
```
```bash
RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 16 32 --wat fib/fib.wat
```
```bash
RUST_LOG=debug cargo run --release -p runner --  --guest "fib" --benchmark-args 16 32 --compress --execution-step-size 100 --memory-step-size 1000
```
