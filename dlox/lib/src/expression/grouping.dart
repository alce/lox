import '../visitor.dart';
import 'base.dart';

class GroupingExpression implements Expression {
  final Expression expression;

  GroupingExpression(this.expression);

  @override
  T accept<T>(Visitor<T> visitor) {
    return visitor.visitGroupingExpression(this);
  }
}
