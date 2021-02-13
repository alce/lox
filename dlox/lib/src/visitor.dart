import 'expression.dart';
import 'statement.dart';

abstract class ExprVisitor<T> {
  T visitBinaryExpr(BinaryExpr expr);
  T visitGroupingExpr(GroupingExpr expr);
  T visitLiteralExpr(LiteralExpr expr);
  T visitUnaryExpr(UnaryExpr expr);
  T visitVariableExpr(VariableExpr expr);
  T visitAssignExpr(AssignExpr expr);
}

abstract class StmtVisitor<T> {
  T visitExpressionStmt(ExpressionStmt stmt);
  T visitPrintStmt(PrintStmt stmt);
  T visitVarStmt(VarStmt stmt);
}
