import 'expression.dart';
import 'token.dart';
import 'visitor.dart';

abstract class Stmt {
  T accept<T>(StmtVisitor<T> visitor);
}

class BlockStmt implements Stmt {
  final List<Stmt> statements;

  BlockStmt(this.statements);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitBlockStmt(this);
}

class ExpressionStmt implements Stmt {
  final Expr expression;

  ExpressionStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitExpressionStmt(this);
}

class IfStmt implements Stmt {
  final Expr condition;
  final Stmt thenBranch;
  final Stmt? elseBranch;

  IfStmt(this.condition, this.thenBranch, this.elseBranch);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitIfStmt(this);
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
