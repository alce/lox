import 'package:charcode/ascii.dart';

bool isDigit(int c) => c >= $0 && c <= $9;

bool isAlpha(int c) {
  final char = c & ~32;
  return ($A <= char && char <= $Z) || char == $underscore;
}

bool isAlphaNumeric(int c) => isAlpha(c) || isDigit(c);
