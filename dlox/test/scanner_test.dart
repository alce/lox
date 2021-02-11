import 'package:dlox/src/scanner.dart';
import 'package:test/test.dart';

void main() {
  group('Scanner', () {
    test('hello', () {
      final source = '''
      print "Hello, world";
      ''';

      final scanner = Scanner(source);
      final tokens = scanner.scan().map((token) => token.display()).toList();
      final expected = ['PRINT', 'STRING(Hello, world)', 'SEMICOLON', 'EOF'];

      expect(tokens, expected);
    });

    test('arithmetic', () {
      final source = '''
      (3+12.3)-5/3*2
      ''';

      final scanner = Scanner(source);
      final tokens = scanner.scan().map((token) => token.display()).toList();
      final expected = [
        'LEFT_PAREN',
        'NUMBER(3.0)',
        'PLUS',
        'NUMBER(12.3)',
        'RIGHT_PAREN',
        'MINUS',
        'NUMBER(5.0)',
        'SLASH',
        'NUMBER(3.0)',
        'STAR',
        'NUMBER(2.0)',
        'EOF'
      ];
      expect(tokens, expected);
    });

    test('comparison', () {
      final source = '''
      {==, != < <= \n > >= . \t !}
      ''';

      final scanner = Scanner(source);
      final tokens = scanner.scan().map((token) => token.display()).toList();
      final expected = [
        'LEFT_BRACE',
        'EQUAL_EQUAL',
        'COMMA',
        'BANG_EQUAL',
        'LESS',
        'LESS_EQUAL',
        'GREATER',
        'GREATER_EQUAL',
        'DOT',
        'BANG',
        'RIGHT_BRACE',
        'EOF'
      ];
      expect(tokens, expected);
    });
  });
}
