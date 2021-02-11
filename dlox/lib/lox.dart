import 'src/scanner.dart';

void run(String source) {
  final scanner = Scanner(source);
  final tokens = scanner.scan();
  print(tokens);
}
