import 'src/scanner.dart';
import 'src/token.dart';

void run(String source) {
  final tokens = runPriv(source);
  print(tokens);
}

List<Token> runPriv(String source) {
  final scanner = Scanner(source);
  return scanner.scan();
}
