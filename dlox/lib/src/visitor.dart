import 'expression.dart';

abstract class Visitor<T> {
  T visitBinaryExpr(BinaryExpr exp);
  T visitGroupingExpr(GroupingExpr exp);
  T visitLiteralExpr(LiteralExpr exp);
  T visitUnaryExpr(UnaryExpr exp);
}
