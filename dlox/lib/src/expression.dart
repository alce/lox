import 'token.dart';
import 'visitor.dart';

abstract class Expr {
  T accept<T>(ExprVisitor<T> visitor);
}

class AssignExpr implements Expr {
  final Token name;
  final Expr value;

  AssignExpr(this.name, this.value);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitAssignExpr(this);
}

class BinaryExpr implements Expr {
  final Expr left;
  final Expr right;
  final Token operator;

  BinaryExpr(this.left, this.operator, this.right);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitBinaryExpr(this);

  @override
  String toString() => 'BIN(${left} ${operator.lexeme} ${right})';
}

class GroupingExpr implements Expr {
  final Expr expression;

  GroupingExpr(this.expression);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitGroupingExpr(this);
}

class LiteralExpr implements Expr {
  final Object value;

  LiteralExpr(this.value);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitLiteralExpr(this);

  @override
  String toString() => 'LIT(${value})';
}

class LogicalExpr implements Expr {
  final Expr left;
  final Token operator;
  final Expr right;

  LogicalExpr(this.left, this.operator, this.right);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitLogicalExpr(this);
}

class UnaryExpr implements Expr {
  final Expr right;
  final Token operator;

  UnaryExpr(this.operator, this.right);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitUnaryExpr(this);

  @override
  String toString() => 'UN(${operator.lexeme} ${right})';
}

class VariableExpr implements Expr {
  final Token name;

  VariableExpr(this.name);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitVariableExpr(this);
}
