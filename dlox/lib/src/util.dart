import 'package:charcode/ascii.dart';

bool isDigit(int c) => c >= $0 && c <= $9;

bool isAlpha(int c) {
  final char = c & ~32;
  return ($A <= char && char <= $Z) || char == $underscore;
}

bool isAlphaNumeric(int c) => isAlpha(c) || isDigit(c);

String stringify(Object? object) {
  if (object == null) {
    return 'nil';
  }

  if (object is double) {
    var text = object.toString();
    if (text.endsWith('.0')) {
      text = text.substring(0, text.length - 2);
    }
    return text;
  }

  return object.toString();
}
