import '../visitor.dart';

abstract class Expression {
  T accept<T>(Visitor<T> visitor);
}
