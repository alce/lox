import 'dart:io';

void main(List<String> args) {
  if (args.length > 1) {
    print('Usage: lox [script]');
    exit(64);
  } else if (args.length == 1) {
    execute(args.first);
  } else {
    repl();
  }
}

void execute(String source) {
  print('execute ${source}');
}

void repl() {
  print('start repl');
}
