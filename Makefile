all: sha256

sha256:
	RUST_BACKTRACE=full cargo test --test test_sha256

bench:
	RUST_BACKTRACE=full cargo bench --bench sha256 --features unstable
