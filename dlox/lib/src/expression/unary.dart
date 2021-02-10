import '../token.dart';
import '../visitor.dart';
import 'base.dart';

class UnaryExpression implements Expression {
  final Expression right;
  final Token operator;

  UnaryExpression(this.right, this.operator);

  @override
  T accept<T>(Visitor<T> visitor) {
    // TODO: implement accept
    throw UnimplementedError();
  }
}
