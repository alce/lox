TOOL_SRC := tool/pubspec.lock $(shell find tool -name '*.dart')
DART_SRC := dlox/pubspec.lock $(shell find dlox -name '*.dart')
RUST_SRC := rlox/Cargo.lock $(shell find rlox -name '*.rs')
C_SRC := $(shell find clox -name '*.c')
SWIFT_SRC := $(shell find slox/Sources -name '*.swift')

DLOX := bin/dlox
RLOX := bin/rlox
CLOX := bin/clox
SLOX := bin/slox
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
	@$(TEST_RUNNER) chap10_functions -i $(RLOX)

test_c: $(CLOX) $(TEST_RUNNER)
	@echo "Testing C VM..."
	@$(TEST_RUNNER) chap22_local -i $(CLOX)

test_swift: $(SLOX) $(TEST_RUNNER)
	@echo "Testing Swift interpreter..."
	@$(TEST_RUNNER) chap04_scanning -i $(SLOX)

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

$(SLOX): $(SWIFT_SRC)
	@mkdir -p bin
	@echo "Compiling Swift interpreter..."
	@cd slox && swift build -c=release > /dev/null
	@cp slox/.build/x86_64-apple-macosx/release/slox bin

$(TEST_RUNNER): $(TOOL_SRC)
	@mkdir -p bin
	@echo "Compiling test runner..."
	@dart compile exe -o $(TEST_RUNNER) tool/bin/test.dart >/dev/null

.PHONY: clean