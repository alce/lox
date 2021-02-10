import 'dart:io';

import 'package:dlox/lox.dart' as lox;

void main(List<String> args) {
  if (args.length > 1) {
    print('Usage: lox [script]');
    exit(64);
  } else if (args.length == 1) {
    final source = File(args.first).readAsStringSync();
    lox.run(source);
  } else {
    repl();
  }
}

void repl() {
  while (true) {
    stdout.write('> ');
    final source = stdin.readLineSync();
    if (source == null) {
      break;
    }

    lox.run(source);
  }
}
