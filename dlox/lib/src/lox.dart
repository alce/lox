import 'dart:io';

import 'exception.dart';
import 'interpreter.dart';
import 'parser.dart';
import 'scanner.dart';
import 'token.dart';

class Lox {
  static final _interpreter = Interpreter();

  static bool _hadError = false;

  static bool _hadRuntimeError = false;

  static void main(List<String> args) {
    if (args.isNotEmpty) {
      try {
        runFile(File(args.first).readAsStringSync());
      } on FileSystemException catch (e) {
        stderr.write('Cannot read "${args.first}". ${e.osError?.message}');
        exit(1);
      }
    } else {
      runPrompt();
    }
  }

  static void runFile(String source) {
    run(source);
    if (_hadError) exit(65);
    if (_hadRuntimeError) exit(70);
  }

  static void runPrompt() {
    while (true) {
      stdout.write('> ');
      final source = stdin.readLineSync();
      if (source == null) break;
      run(source);
      _hadError = false;
    }
  }

  static void run(String source) {
    final scanner = Scanner(source);
    final parser = Parser(scanner.scan());
    final statements = parser.parse();

    if (_hadError) return;

    _interpreter.interpret(statements);
  }

  static void scanError(int line, String message) {
    _report(line, message);
  }

  static void error(Token token, String message) {
    if (token.type == TokenType.EOF) {
      _report(token.line, message, ' at end');
    } else {
      _report(token.line, message, " at '${token.lexeme}'");
    }
  }

  static void runtimeError(RuntimeError error) {
    stderr.writeln('${error}\n[line ${error.token.line}]');
    _hadRuntimeError = true;
  }

  static void _report(int line, String message, [where = '']) {
    stderr.writeln('[line ${line}] Error${where}: ${message}');
    _hadError = true;
  }
}
