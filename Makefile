BIN_DIR := bin
TOOL_SOURCES := tool/pubspec.lock $(shell find tool -name '*.dart')
INTERPRETER_SOURCES := dlox/pubspec.lock $(shell find dlox -name '*.dart')
VM_SOURCES := rlox/Cargo.lock $(shell find rlox -name '*.rs')
VM := $(BIN_DIR)/rlox
INTERPRETER := $(BIN_DIR)/dlox
TEST_RUNNER := $(BIN_DIR)/test_runner

default: test_vm

clean:
	@rm -rf $(BIN_DIR)

test: $(INTERPRETER) $(TEST_RUNNER)
	@echo "Testing interpreter..."
	@bin/test_runner jlox -i bin/dlox

test_vm: $(VM) $(TEST_RUNNER)
	@echo "Testing virtual machine..."
	@bin/test_runner chap17_compiling -i bin/rlox

$(VM): $(VM_SOURCES)
	@mkdir -p bin
	@echo "Compiling virtual machine..."
	@cd rlox && cargo -q build --release
	@cp rlox/target/release/rlox bin

$(INTERPRETER): $(INTERPRETER_SOURCES)
	@mkdir -p bin
	@echo "Compiling interpreter..."
	@dart compile exe -o bin/dlox dlox/bin/lox.dart >/dev/null

$(TEST_RUNNER): $(TOOL_SOURCES)
	@mkdir -p bin
	@echo "Compiling test runner..."
	@dart compile exe -o bin/test_runner tool/bin/test.dart >/dev/null

.PHONY: clean