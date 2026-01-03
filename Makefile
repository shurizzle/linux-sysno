generate:
	cd sysno-gen && \
		cargo build --release && \
		cargo run --release -- ../src

clean:
	cd sysno-gen && cargo clean

.PHONY: generate clean
