TOOL_SRC := tool/pubspec.lock $(shell find tool -name '*.dart')
DART_SRC := dlox/pubspec.lock $(shell find dlox -name '*.dart')
RUST_SRC := rlox/Cargo.lock $(shell find rlox -name '*.rs')
C_SRC := $(shell find clox -name '*.c')

DLOX := bin/dlox
RLOX := bin/rlox
CLOX := bin/clox
TEST_RUNNER := bin/test_runner

CFLAGS := -std=c99 -O3 -flto -Wall -Wextra -Werror -Wno-unused-parameter

all: test_dart test_c test_rust

clean:
	@rm -rf bin

test_dart: $(DLOX) $(TEST_RUNNER)
	@echo "Testing Dart interpreter..."
	@$(TEST_RUNNER) jlox -i $(DLOX)

test_rust: $(RLOX) $(TEST_RUNNER)
	@echo "Testing Rust VM..."
	@$(TEST_RUNNER) chap07_evaluating -i $(RLOX)

test_c: $(CLOX) $(TEST_RUNNER)
	@echo "Testing C VM..."
	@$(TEST_RUNNER) chap22_local -i $(CLOX)

$(DLOX): $(DART_SRC)
	@mkdir -p bin
	@echo "Compiling Dart interpreter..."
	@dart compile exe -o $(DLOX) dlox/bin/lox.dart >/dev/null

$(RLOX): $(RUST_SRC)
	@mkdir -p bin
	@echo "Compiling Rust VM..."
	@cd rlox && cargo -q build --release
	@cp rlox/target/release/rlox bin

$(CLOX): $(C_SRC)
	@mkdir -p bin
	@echo "Compiling C VM..."
	@$(CC) $(CFLAGS) $(C_SRC) -o bin/clox

$(TEST_RUNNER): $(TOOL_SRC)
	@mkdir -p bin
	@echo "Compiling test runner..."
	@dart compile exe -o $(TEST_RUNNER) tool/bin/test.dart >/dev/null

.PHONY: clean