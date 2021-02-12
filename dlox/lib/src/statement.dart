import 'expression.dart';
import 'token.dart';
import 'visitor.dart';

abstract class Stmt {
  T accept<T>(StmtVisitor<T> visitor);
}

class ExpressionStmt implements Stmt {
  final Expr expression;

  ExpressionStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitExpressionStmt(this);
}

class PrintStmt implements Stmt {
  final Expr expression;

  PrintStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitPrintStmt(this);
}

class VarStmt implements Stmt {
  final Token name;
  final Expr? initializer;

  VarStmt(this.name, this.initializer);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitVarStmt(this);
}
