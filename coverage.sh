# stable rustc 1.43.1 can't build coverage, use nightly instead - `rustup default nightly`
# for using in pipeline (maybe), in CLion with Rust plugin coverage doesn't work - https://github.com/intellij-rust/intellij-rust/pull/5356
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
cargo build
cargo test
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
