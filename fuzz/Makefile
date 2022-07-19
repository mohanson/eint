
# refrence: https://github.com/nervosnetwork/ckb/tree/develop/script/fuzz

TARGET ?= compare
JOBS ?= 40
TARGET_TRIPLE ?= x86_64-unknown-linux-gnu
NAME ?= eint

run:
	cargo fuzz run -j $(JOBS) $(TARGET)

report:
	cargo fuzz coverage $(TARGET)
	cargo cov -- show target/$(TARGET_TRIPLE)/release/$(TARGET) --Xdemangler=rustfilt --format=html -instr-profile=coverage/$(TARGET)/coverage.profdata --name=$(NAME) --line-coverage-gt=1> /tmp/$(NAME)-coverage-report.html

install-tools:
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
	cargo install rustfilt
