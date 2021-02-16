BUILD_DIR := build
TOOL_SOURCES := tool/pubspec.lock $(shell find tool -name '*.dart')
INTERPRETER_SOURCES := dlox/pubspec.lock $(shell find dlox -name '*.dart')
INTERPRETER := $(BUILD_DIR)/dlox
TEST_RUNNER := $(BUILD_DIR)/test_runner

default: test

clean:
	@rm -rf $(BUILD_DIR)

test: $(INTERPRETER) $(TEST_RUNNER)
	@echo "Testing interpreter..."
	@build/test_runner jlox -i build/dlox

test_dlox: $(INTERPRETER) $(TEST_RUNNER)
	@echo "Testing interpreter..."
	@build/test_runner jlox -i build/dlox

$(INTERPRETER): $(INTERPRETER_SOURCES)
	@mkdir -p build
	@echo "Compiling interpreter..."
	@dart compile exe -o build/dlox dlox/bin/lox.dart >/dev/null

$(TEST_RUNNER): $(TOOL_SOURCES)
	@mkdir -p build
	@echo "Compiling test runner..."
	@dart compile exe -o build/test_runner tool/bin/test.dart >/dev/null

.PHONY: clean