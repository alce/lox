import 'package:dlox/lox.dart';
import 'package:test/test.dart';

void main() {
  test('smoke', () {
    final source = '''
    print "Hello, world";
    ''';
    final tokens = runPriv(source);

    // Tokens PRINT, STRING, SEMICOLON, EOF
    expect(tokens.length, 4);
  });
}
