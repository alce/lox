import 'expression.dart';

abstract class Visitor<T> {
  T visitBinaryExpr(BinaryExpr expr);
  T visitGroupingExpr(GroupingExpr expr);
  T visitLiteralExpr(LiteralExpr expr);
  T visitUnaryExpr(UnaryExpr expr);
}
