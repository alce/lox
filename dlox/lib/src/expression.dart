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
}

class CallExpr implements Expr {
  final Expr callee;
  final Token paren;
  final List<Expr> arguments;

  CallExpr(this.callee, this.paren, this.arguments);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitCallExpr(this);
}

class GetExpr implements Expr {
  final Token name;
  final Expr object;

  GetExpr(this.name, this.object);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitGetExpr(this);
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
}

class LogicalExpr implements Expr {
  final Expr left;
  final Token operator;
  final Expr right;

  LogicalExpr(this.left, this.operator, this.right);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitLogicalExpr(this);
}

class SetExpr implements Expr {
  final Expr object;
  final Token name;
  final Expr value;

  SetExpr(this.object, this.name, this.value);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitSetExpr(this);
}

class SuperExpr implements Expr {
  final Token keyword;
  final Token method;

  SuperExpr(this.keyword, this.method);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitSuperExpr(this);
}

class ThisExpr implements Expr {
  final Token keyword;

  ThisExpr(this.keyword);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitThisExpr(this);
}

class UnaryExpr implements Expr {
  final Expr right;
  final Token operator;

  UnaryExpr(this.operator, this.right);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitUnaryExpr(this);
}

class VariableExpr implements Expr {
  final Token name;

  VariableExpr(this.name);

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitVariableExpr(this);
}
