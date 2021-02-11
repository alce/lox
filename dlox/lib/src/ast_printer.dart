import 'expression.dart';
import 'visitor.dart';

class AstPrinter implements Visitor<String> {
  String print(Expr expr) {
    return expr.accept(this);
  }

  @override
  String visitBinaryExpr(BinaryExpr exp) {
    return _parenthesize(exp.operator.lexeme, [exp.left, exp.right]);
  }

  @override
  String visitGroupingExpr(GroupingExpr exp) {
    return _parenthesize('group', [exp.expression]);
  }

  @override
  String visitLiteralExpr(LiteralExpr exp) {
    return exp.value.toString();
  }

  @override
  String visitUnaryExpr(UnaryExpr exp) {
    return _parenthesize(exp.operator.lexeme, [exp.right]);
  }

  String _parenthesize(String name, List<Expr> expressions) {
    final buf = StringBuffer();

    buf.write('(');
    buf.write(name);

    for (final exp in expressions) {
      buf.write(' ');
      buf.write(exp.accept(this));
    }

    buf.write(')');

    return buf.toString();
  }
}
