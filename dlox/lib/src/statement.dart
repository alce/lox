import 'expression.dart';
import 'visitor.dart';

abstract class Stmt {
  T accept<T>(StmtVisitor<T> visitor);
}

class ExpressionStmt implements Stmt {
  final Expr expression;

  ExpressionStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) {
    return visitor.visitExpressionStmt(this);
  }
}

class PrintStmt implements Stmt {
  final Expr expression;

  PrintStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) {
    return visitor.visitPrintStmt(this);
  }
}
