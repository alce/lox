import 'expression.dart';
import 'visitor.dart';

class AstPrinter implements Visitor<String> {
  String print(Expr expr) {
    return expr.accept(this);
  }

  @override
  String visitBinaryExpr(BinaryExpr expr) {
    return _parenthesize(expr.operator.lexeme, [expr.left, expr.right]);
  }

  @override
  String visitGroupingExpr(GroupingExpr expr) {
    return _parenthesize('group', [expr.expression]);
  }

  @override
  String visitLiteralExpr(LiteralExpr expr) {
    return expr.value.toString();
  }

  @override
  String visitUnaryExpr(UnaryExpr expr) {
    return _parenthesize(expr.operator.lexeme, [expr.right]);
  }

  String _parenthesize(String name, List<Expr> expressions) {
    final buf = StringBuffer();

    buf.write('(');
    buf.write(name);

    for (final expr in expressions) {
      buf.write(' ');
      buf.write(expr.accept(this));
    }

    buf.write(')');

    return buf.toString();
  }
}
