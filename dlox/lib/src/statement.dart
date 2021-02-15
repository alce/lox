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

class ClassStmt implements Stmt {
  final Token name;
  final VariableExpr? superclass;
  final List<FunctionStmt> methods;

  ClassStmt(this.name, this.superclass, this.methods);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitClassStmt(this);
}

class ExpressionStmt implements Stmt {
  final Expr expression;

  ExpressionStmt(this.expression);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitExpressionStmt(this);
}

class FunctionStmt implements Stmt {
  final Token name;
  final List<Token> params;
  final List<Stmt> body;

  FunctionStmt(this.name, this.params, this.body);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitFunctionStmt(this);
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

class ReturnStmt implements Stmt {
  final Token keyword;
  final Expr value;

  ReturnStmt(this.keyword, this.value);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitReturnStmt(this);
}

class VarStmt implements Stmt {
  final Token name;
  final Expr? initializer;

  VarStmt(this.name, this.initializer);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitVarStmt(this);
}

class WhileStmt implements Stmt {
  final Expr condition;
  final Stmt body;

  WhileStmt(this.condition, this.body);

  @override
  T accept<T>(StmtVisitor<T> visitor) => visitor.visitWhileStmt(this);
}
