import 'dart:io';

import 'package:charcode/ascii.dart';
import 'package:dlox/src/token.dart';

// TODO
void error(int line, String message) {
  print('ERROR: [${line}] ${message}');
  exit(1);
}

class Scanner {
  final String _source;

  final _tokens = <Token>[];

  int _idx = 0;
  int _start = 0;
  int _line = 1;

  static final _keywords = <String, TokenType>{
    'and': TokenType.AND,
    'class': TokenType.CLASS,
    'else': TokenType.ELSE,
    'false': TokenType.FALSE,
    'for': TokenType.FOR,
    'fun': TokenType.FUN,
    'if': TokenType.IF,
    'nil': TokenType.NIL,
    'or': TokenType.OR,
    'print': TokenType.PRINT,
    'return': TokenType.RETURN,
    'super': TokenType.SUPER,
    'this': TokenType.THIS,
    'true': TokenType.TRUE,
    'var': TokenType.VAR,
    'while': TokenType.WHILE,
  };

  Scanner(this._source);

  bool get _isAtEnd => _idx >= _source.length;

  List<Token> scan() {
    while (!_isAtEnd) {
      _start = _idx;
      _scanToken();
    }
    _tokens.add(Token(TokenType.EOF, '', null, _line));
    return _tokens;
  }

  void _scanToken() {
    final c = _advance();

    switch (c) {
      case $lparen:
        _addToken(TokenType.LEFT_PAREN);
        break;
      case $rparen:
        _addToken(TokenType.RIGHT_PAREN);
        break;
      case $lbrace:
        _addToken(TokenType.LEFT_BRACE);
        break;
      case $rbrace:
        _addToken(TokenType.RIGHT_BRACE);
        break;
      case $comma:
        _addToken(TokenType.COMMA);
        break;
      case $dot:
        _addToken(TokenType.DOT);
        break;
      case $minus:
        _addToken(TokenType.MINUS);
        break;
      case $plus:
        _addToken(TokenType.PLUS);
        break;
      case $semicolon:
        _addToken(TokenType.SEMICOLON);
        break;
      case $asterisk:
        _addToken(TokenType.STAR);
        break;
      case $exclamation:
        _addToken(_match($equal) ? TokenType.BANG_EQUAL : TokenType.BANG);
        break;
      case $equal:
        _addToken(_match($equal) ? TokenType.EQUAL_EQUAL : TokenType.EQUAL);
        break;
      case $less_than:
        _addToken(_match($equal) ? TokenType.LESS_EQUAL : TokenType.LESS);
        break;
      case $greater_than:
        _addToken(_match($equal) ? TokenType.GREATER_EQUAL : TokenType.GREATER);
        break;
      case $slash:
        if (_match($slash)) {
          while (_peek() != $lf && !_isAtEnd) {
            _advance();
          }
        } else {
          _addToken(TokenType.SLASH);
        }
        break;
      case $space:
      case $cr:
      case $tab:
        break;
      case $lf:
        _line++;
        break;
      case $double_quote:
        _string();
        break;
      default:
        if (_isDigit(c)) {
          _number();
        } else if (_isAlpha(c)) {
          _identifier();
        } else {
          error(_line, 'Unexpected character');
        }
        break;
    }
  }

  void _string() {
    while (_peek() != $double_quote && !_isAtEnd) {
      if (_peek() == $lf) {
        _line++;
      }
      _advance();
    }

    if (_isAtEnd) {
      error(_line, 'Unterminated string.');
      return;
    }

    _advance();
    _addToken(TokenType.STRING, _source.substring(_start + 1, _idx - 1));
  }

  void _number() {
    while (_isDigit(_peek())) {
      _advance();
    }

    if (_peek() == $dot && _isDigit(_peekNext())) {
      _advance();
    }

    while (_isDigit(_peek())) {
      _advance();
    }

    _addToken(TokenType.NUMBER, double.parse(_source.substring(_start, _idx)));
  }

  void _identifier() {
    while (_isAlphaNumeric(_peek())) {
      _advance();
    }

    _addToken(_keywords[_source.substring(_start, _idx)] ?? TokenType.IDENT);
  }

  int _advance() {
    _idx++;
    return _source.codeUnitAt(_idx - 1);
  }

  void _addToken(TokenType type, [Object? literal]) {
    final text = _source.substring(_start, _idx);
    _tokens.add(Token(type, text, literal, _line));
  }

  bool _match(int expected) {
    if (_isAtEnd || (_source.codeUnitAt(_idx) != expected)) {
      return false;
    }
    _idx++;
    return true;
  }

  int _peek() {
    if (_isAtEnd) {
      return $nul;
    }
    return _source.codeUnitAt(_idx);
  }

  int _peekNext() {
    if (_idx + 1 >= _source.length) {
      return $nul;
    }

    return _source.codeUnitAt(_idx + 1);
  }

  bool _isDigit(int c) => c >= $0 && c <= $9;

  bool _isAlpha(int c) {
    final char = c & ~32;
    return ($A <= char && char <= $Z) || char == $underscore;
  }

  bool _isAlphaNumeric(int c) => _isAlpha(c) || _isDigit(c);
}
