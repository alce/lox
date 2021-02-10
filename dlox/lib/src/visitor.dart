import 'expression/expression.dart';

abstract class Visitor<T> {
  T visitBinaryExpression(BinaryExpression exp);
  T visitGroupingExpression(GroupingExpression exp);
  T visitLiteralExpression(LiteralExpression exp);
  T visitUnaryExpression(UnaryExpression exp);
}
