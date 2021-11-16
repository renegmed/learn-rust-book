init:
	cargo new tests --lib

test:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1 --show-output

test_100:
	cargo test one_hundred

test_add:
	cargo test add 

# run only the [ignored] tests
test_ignored:
	cargo test -- --ignored 