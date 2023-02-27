all: sha256

sha256:
	RUST_BACKTRACE=full cargo test --test test_sha256
