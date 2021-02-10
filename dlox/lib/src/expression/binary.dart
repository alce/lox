import '../token.dart';
import '../visitor.dart';
import 'base.dart';

class BinaryExpression implements Expression {
  final Expression left;
  final Expression right;
  final Token operator;

  BinaryExpression(this.left, this.right, this.operator);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitBinaryExpression(this);
  }
}
