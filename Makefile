all: sha256

sha256:
	RUST_BACKTRACE=full cargo +nightly bench --bench sha256
