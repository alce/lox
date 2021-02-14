import 'expression.dart';
import 'visitor.dart';

class Nil implements Expr {
  @override
  bool operator ==(Object other) {
    return other is Nil;
  }

  @override
  String toString() => 'nil';

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitNil(this);
}
