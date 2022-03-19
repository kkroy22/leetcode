b:
	hyperfine cargo run >> benchmark.txt
p:
	cargo flamegraph --dev