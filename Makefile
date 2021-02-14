BUILD_DIR := build
TOOL_SOURCES := tool/pubspec.lock $(shell find tool -name '*.dart')
TEST_BIN := $(BUILD_DIR)/lox_test

default: test

clean:
	@rm -rf $(BUILD_DIR)

test: dlox $(TEST_BIN)
	@build/lox_test chap04_scanning -i build/dlox

dlox:
	@mkdir -p build
	@echo "Compiling dlox..."
	@dart compile exe -o build/dlox dlox/bin/lox.dart >/dev/null

$(TEST_BIN): $(TOOL_SOURCES)
	@mkdir -p build
	@echo "Compiling lox_test..."
	@dart compile exe -o build/lox_test tool/bin/test.dart >/dev/null

.PHONY: dlox clean