import 'expression.dart';
import 'nil.dart';
import 'statement.dart';

abstract class ExprVisitor<T> {
  T visitAssignExpr(AssignExpr expr);
  T visitBinaryExpr(BinaryExpr expr);
  T visitCallExpr(CallExpr expr);
  T visitGroupingExpr(GroupingExpr expr);
  T visitLogicalExpr(LogicalExpr expr);
  T visitLiteralExpr(LiteralExpr expr);
  T visitUnaryExpr(UnaryExpr expr);
  T visitNil(Nil expr);
  T visitVariableExpr(VariableExpr expr);
}

abstract class StmtVisitor<T> {
  T visitBlockStmt(BlockStmt stmt);
  T visitExpressionStmt(ExpressionStmt stmt);
  T visitFunctionStmt(FunctionStmt stmt);
  T visitIfStmt(IfStmt stmt);
  T visitPrintStmt(PrintStmt stmt);
  T visitReturnStmt(ReturnStmt stmt);
  T visitVarStmt(VarStmt stmt);
  T visitWhileStmt(WhileStmt stmt);
}
