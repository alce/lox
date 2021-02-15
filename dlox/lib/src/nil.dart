import 'expression.dart';
import 'visitor.dart';

class Nil implements Expr {
  static final _nil = Nil._();

  factory Nil() => _nil;

  Nil._();

  @override
  bool operator ==(Object other) => other is Nil;

  @override
  String toString() => 'nil';

  @override
  T accept<T>(ExprVisitor<T> visitor) => visitor.visitNil(this);
}
