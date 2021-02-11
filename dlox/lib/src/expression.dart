import 'token.dart';
import 'visitor.dart';

abstract class Expr {
  T accept<T>(Visitor<T> visitor);
}

class BinaryExpr implements Expr {
  final Expr left;
  final Expr right;
  final Token operator;

  BinaryExpr(this.left, this.operator, this.right);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitBinaryExpr(this);
  }

  @override
  String toString() => 'BIN(${left} ${operator.lexeme} ${right})';
}

class GroupingExpr implements Expr {
  final Expr expression;

  GroupingExpr(this.expression);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitGroupingExpr(this);
  }
}

class LiteralExpr implements Expr {
  final Object? value;

  LiteralExpr(this.value);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitLiteralExpr(this);
  }

  @override
  String toString() => 'LIT(${value})';
}

class UnaryExpr implements Expr {
  final Expr right;
  final Token operator;

  UnaryExpr(this.operator, this.right);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitUnaryExpr(this);
  }

  @override
  String toString() => 'UN(${operator.lexeme} ${right})';
}
