generate:
	cd sysno-gen && \
		cargo build --release && \
		cargo run --release -- ../src
