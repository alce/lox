import 'dart:io';

import 'package:dlox/lox.dart';

void main(List<String> args) {
  if (args.length > 1) {
    print('Usage: lox [script]');
    exit(64);
  }

  Lox.main(args);
}
