BUILD_DIR := build
TOOL_SOURCES := tool/pubspec.lock $(shell find tool -name '*.dart')
DLOX_SOURCES := dlox/pubspec.lock $(shell find dlox -name '*.dart')
DLOX_BIN := $(BUILD_DIR)/dlox
TEST_BIN := $(BUILD_DIR)/lox_test

default: test

clean:
	@rm -rf $(BUILD_DIR)

test: $(DLOX_BIN) $(TEST_BIN)
	@build/lox_test chap10_functions -i build/dlox

$(DLOX_BIN): $(DLOX_SOURCES)
	@mkdir -p build
	@echo "Compiling dart binary..."
	@dart compile exe -o build/dlox dlox/bin/lox.dart >/dev/null

$(TEST_BIN): $(TOOL_SOURCES)
	@mkdir -p build
	@echo "Compiling test binary..."
	@dart compile exe -o build/lox_test tool/bin/test.dart >/dev/null

.PHONY: clean