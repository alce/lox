import 'dart:io';

import 'package:dlox/lox.dart' as lox;
import 'package:dlox/src/interpreter.dart';

void main(List<String> args) {
  if (args.length > 1) {
    print('Usage: lox [script]');
    exit(64);
  } else if (args.length == 1) {
    try {
      final source = File(args.first).readAsStringSync();
      final interpreter = Interpreter();
      lox.run(source, interpreter);
    } on lox.LoxException catch (e) {
      print(e);
    } on FileSystemException catch (e) {
      print('Cannot read "${args.first}". ${e.osError?.message}');
    }
  } else {
    repl();
  }
}

void repl() {
  final interpreter = Interpreter();

  while (true) {
    stdout.write('> ');
    final source = stdin.readLineSync();
    if (source == null) {
      break;
    }

    try {
      lox.run(source, interpreter);
    } catch (e) {
      print(e);
    }
  }
}
