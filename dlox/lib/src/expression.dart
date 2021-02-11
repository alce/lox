import 'token.dart';
import 'visitor.dart';

abstract class Expression {
  T accept<T>(Visitor<T> visitor);
}

class BinaryExpression implements Expression {
  final Expression left;
  final Expression right;
  final Token operator;

  BinaryExpression(this.left, this.operator, this.right);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitBinaryExpression(this);
  }

  @override
  String toString() => 'BIN(${left} ${operator.lexeme} ${right})';
}

class GroupingExpression implements Expression {
  final Expression expression;

  GroupingExpression(this.expression);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitGroupingExpression(this);
  }
}

class LiteralExpression implements Expression {
  final Object? value;

  LiteralExpression(this.value);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitLiteralExpression(this);
  }

  @override
  String toString() => 'LIT(${value})';
}

class UnaryExpression implements Expression {
  final Expression right;
  final Token operator;

  UnaryExpression(this.operator, this.right);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitUnaryExpression(this);
  }

  @override
  String toString() => 'UN(${operator.lexeme} ${right})';
}
