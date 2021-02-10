import '../visitor.dart';
import 'base.dart';

class LiteralExpression implements Expression {
  final Object? value;

  LiteralExpression(this.value);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitLiteralExpression(this);
  }
}
